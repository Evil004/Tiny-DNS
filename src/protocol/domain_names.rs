#[derive(Debug,Clone)]
#[allow(dead_code)]
pub struct DomainNames {
    labels_array: Vec<Label>,
}

impl DomainNames {
    pub fn new(labels_array: Vec<Label>) -> DomainNames {
        return DomainNames { labels_array };
    }

    pub fn get_labels(&self) -> Vec<Label> {
        return self.labels_array.clone();
    }

    #[allow(dead_code)]
    pub fn get_domains(&self) -> Vec<String> {
        let mut domains = Vec::new();
        let mut domain = String::new();

        for label in self.labels_array.iter() {
            domain.push_str(&label.string);
            match label.next {
                Next::Label => {
                    domain.push_str(".");
                }
                Next::End => {
                    domains.push(domain.clone());
                    domain.clear();
                }
                Next::Pointer { pos } => {
                    let labels = self.get_from_pos(pos).to_vec();

                    let pointer_domain = self.get_domain_for_pointer(labels);
                    domain.push_str(".");
                    domain.push_str(&pointer_domain);
                    domains.push(domain.clone());
                    domain.clear();
                }
            }
        }

        return domains;
    }

    fn get_domain_for_pointer(&self, label_vec: Vec<Label>) -> String {
        let mut domains = Vec::new();
        let mut domain = String::new();

        for label in label_vec {
            domain.push_str(&label.string);
            match label.next {
                Next::Label => {
                    domain.push_str(".");
                }
                Next::End => {
                    domains.push(domain.clone());
                    domain.clear();
                    break;
                }
                Next::Pointer { pos } => {
                    let labels = self.get_from_pos(pos).to_vec();
                    let pointer_domain = self.get_domain_for_pointer(labels);

                    domain.push_str(".");
                    domain.push_str(&pointer_domain);
                    domains.push(domain.clone());
                    break;
                }
            }
        }
        return domains.join("");
    }

    fn get_from_pos(&self, pos: u16) -> &[Label] {
        let mut reading_pos = 0;
        for (i, label) in self.labels_array.iter().enumerate() {
            if pos != reading_pos {
                reading_pos += label.len as u16 + 1;
            } else {
                return &self.labels_array[i..];
            }
        }

        return &[];
    }
}

#[derive(Clone, Debug)]
pub struct Label {
    pub len: u8,
    pub string: String,
    pub next: Next,
}

impl Label {
    pub fn new(len: u8, string: String, next: Next) -> Label {
        return Label {
            len: len,
            string: string,
            next: next,
        };
    }
}

#[derive(Clone, Debug)]
pub enum Next {
    Pointer { pos: u16 },
    Label,
    End,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_domain() {
        let domain_names = DomainNames {
            labels_array: vec![
                Label {
                    len: 3,
                    string: "www".to_string(),
                    next: Next::Label,
                },
                Label {
                    len: 6,
                    string: "google".to_string(),
                    next: Next::Label,
                },
                Label {
                    len: 3,
                    string: "com".to_string(),
                    next: Next::End,
                },
                Label {
                    len: 6,
                    string: "images".to_string(),
                    next: Next::Pointer { pos: 4 },
                },
                Label {
                    len: 4,
                    string: "test".to_string(),
                    next: Next::Pointer { pos: 4 },
                },
            ],
        };

        let domains = domain_names.get_domains();

        assert_eq!(
            domains,
            vec!["www.google.com", "images.google.com", "test.google.com"]
        );
    }

    #[test]
    fn test_get_domain_with_pointer_to_pointer() {
        let domain_names = DomainNames {
            labels_array: vec![
                Label {
                    len: 3,
                    string: "www".to_string(),
                    next: Next::Label,
                },
                Label {
                    len: 6,
                    string: "google".to_string(),
                    next: Next::Label,
                },
                Label {
                    len: 3,
                    string: "com".to_string(),
                    next: Next::End,
                },
                Label {
                    len: 6,
                    string: "images".to_string(),
                    next: Next::Pointer { pos: 4 },
                },
                Label {
                    len: 4,
                    string: "test".to_string(),
                    next: Next::Pointer { pos: 15 },
                },
            ],
        };

        let domains = domain_names.get_domains();

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
