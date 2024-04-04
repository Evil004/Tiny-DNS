#[derive(Debug, Clone)]
#[allow(dead_code)]

pub struct DomainNames {
    labels_array: Vec<DomainParts>,
}

#[allow(dead_code)]
impl DomainNames {
    pub fn new_from_vec(labels_array: Vec<DomainParts>) -> DomainNames {
        return DomainNames { labels_array };
    }

    pub fn get_labels(&self) -> Vec<DomainParts> {
        return self.labels_array.clone();
    }

    pub fn get_total_len(&self) -> usize {
        let mut total_len = 0;
        for label in self.labels_array.iter() {
            total_len += label.get_size() as usize
        }
        return total_len;
    }

    pub fn get_domains(&self) -> Vec<String> {
        let mut domains = Vec::new();
        let mut domain = String::new();

        for label in self.labels_array.iter() {
            match label {
                DomainParts::Pointer { pos } => {
                    dbg!("Outer Pointer",pos);
                    domain.push_str(&self.get_domain_from_pos(*pos));
                    domains.push(domain.clone());
                    domain.clear();
                }
                DomainParts::Label { len: _, string } => {
                    domain.push_str(&format!("{}.", string));
                }
                DomainParts::End => {
                    domain.pop();
                    domains.push(domain.clone());
                    domain.clear();
                }
            }
        }

        return domains;
    }

    fn get_domain_from_pos(&self, pos: u16) -> String {
        let mut domain = String::new();

        let labels = self.get_from_pos(pos);
        for label in labels {
            dbg!(label);

            match label {
                DomainParts::Pointer { pos } => {
                    dbg!("Inner Pointer",pos);

                    domain.push_str(&self.get_domain_from_pos(*pos));
                    return domain;
                }
                DomainParts::Label { len: _, string } => {
                    domain.push_str(&format!("{}.", string));
                }
                DomainParts::End => {
                    domain.pop();
                    return domain;
                }
            }
        }

        return domain;
    }

    fn get_from_pos(&self, pos: u16) -> &[DomainParts] {
        let mut reading_pos = 0;

        for (i, label) in self.labels_array.iter().enumerate() {
            dbg!(label, reading_pos, pos);
            if pos > reading_pos {
                reading_pos += label.get_size()
            } else {
                dbg!(i, &self.labels_array[i..]);
                return &self.labels_array[i..];
            }
        }

        return &[];
    }
}

#[derive(Clone, Debug)]
pub enum DomainParts {
    Pointer { pos: u16 },
    Label { len: u8, string: String },
    End,
}

impl DomainParts {
    pub fn get_size(&self) -> u16 {
        match self {
            DomainParts::Pointer { .. } => 2,
            DomainParts::Label { len, .. } => *len as u16 + 1,
            DomainParts::End => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_domain() {
        let domain_names = DomainNames {
            labels_array: vec![
                DomainParts::Label {
                    len: 3,
                    string: String::from("www"),
                },
                DomainParts::Label {
                    len: 6,
                    string: String::from("google"),
                },
                DomainParts::Label {
                    len: 3,
                    string: String::from("com"),
                },
                DomainParts::End,
            ],
        };

        let domains = domain_names.get_domains();

        assert_eq!(domains, vec!["www.google.com"]);
    }

    #[test]
    fn test_get_domain_wiht_pointer() {
        let domain_names = DomainNames {
            labels_array: vec![
                DomainParts::Label {
                    len: 3,
                    string: String::from("www"),
                },
                DomainParts::Label {
                    len: 6,
                    string: String::from("google"),
                },
                DomainParts::Label {
                    len: 3,
                    string: String::from("com"),
                },
                DomainParts::End,
                DomainParts::Label {
                    len: 6,
                    string: String::from("images"),
                },
                DomainParts::Pointer { pos: 4 },
            ],
        };

        let domains = domain_names.get_domains();

        assert_eq!(domains, vec!["www.google.com", "images.google.com"]);
    }

    #[test]
    fn test_get_domain_with_pointer_to_pointer() {
        let domain_names = DomainNames {
            labels_array: vec![
                DomainParts::Label {
                    len: 3,
                    string: String::from("www"),
                },
                DomainParts::Label {
                    len: 6,
                    string: String::from("google"),
                },
                DomainParts::Label {
                    len: 3,
                    string: String::from("com"),
                },
                DomainParts::End,
                DomainParts::Label {
                    len: 6,
                    string: String::from("images"),
                },
                DomainParts::Pointer { pos: 4 },
                DomainParts::Label {
                    len: 4,
                    string: String::from("test"),
                },
                DomainParts::Pointer { pos: 16 },
            ],
        };
        dbg!("Before get_domains");

        let domains = domain_names.get_domains();
        dbg!("After get_domains");

        assert_eq!(
            domains,
            vec![
                "www.google.com",
                "images.google.com",
                "test.images.google.com"
            ]
        );
    }
}
