#[derive(Debug, Clone)]
#[allow(dead_code)]

pub struct DomainNames {
    labels_array: Vec<DomainParts>,
    starting_byte: u16,
}

#[allow(dead_code)]
impl DomainNames {
    pub fn new_from_vec_with_starting_point(
        labels_array: Vec<DomainParts>,
        starting_byte: u16,
    ) -> DomainNames {
        return DomainNames {
            labels_array,
            starting_byte,
        };
    }

    pub fn new_from_vec(labels_array: Vec<DomainParts>) -> DomainNames {
        DomainNames::new_from_vec_with_starting_point(labels_array, 0)
    }

    pub fn set_starting_byte(&mut self, starting_byte: u16) {
        self.starting_byte = starting_byte;
    }

    pub fn get_starting_byte(&self) -> u16 {
        return self.starting_byte;
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

    pub fn get_domains(&self) -> Vec<(String, u16)> {
        let mut domains = Vec::new();
        let mut domain = String::new();
        let mut start_pos = 0;

        for (i, label) in self.labels_array.iter().enumerate() {
            if domain.len() == 0 {
                start_pos = self.get_pos_from_index(i);
            }
            match label {
                DomainParts::Pointer { pos } => {
                    domain.push_str(&self.get_domain_from_pos(*pos));
                    domains.push((domain.clone(), start_pos));
                    domain.clear();
                }
                DomainParts::Label { len: _, string } => {
                    domain.push_str(&format!("{}.", string));
                }
                DomainParts::End => {
                    domain.pop();
                    domains.push((domain.clone(), start_pos));
                    domain.clear();
                }
            }
        }

        return domains;
    }

    fn get_pos_from_index(&self, index: usize) -> u16 {
        let mut reading_pos = 0;

        for (i, label) in self.labels_array.iter().enumerate() {
            if i == index {
                return reading_pos;
            }
            reading_pos += label.get_size();
        }

        return reading_pos;
    }

    fn get_domain_from_pos(&self, pos: u16) -> String {
        let mut domain = String::new();

        let labels = self.get_from_pos(pos);
        for label in labels {
            match label {
                DomainParts::Pointer { pos } => {
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
            if pos > reading_pos {
                reading_pos += label.get_size()
            } else {
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
            starting_byte: 12,
        };

        let domains = domain_names.get_domains();

        assert_eq!(domains, vec![(String::from("www.google.com"), 0u16)]);
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
            starting_byte: 12,
        };

        let domains = domain_names.get_domains();

        assert_eq!(
            domains,
            vec![
                (String::from("www.google.com"), 0u16),
                (String::from("images.google.com"), 16u16)
            ]
        );
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
            starting_byte: 12,
        };

        let domains = domain_names.get_domains();

        assert_eq!(
            domains,
            vec![
                (String::from("www.google.com"), 0u16),
                (String::from("images.google.com"), 16u16),
                (String::from("test.images.google.com"), 25u16)
            ]
        );
    }
}
