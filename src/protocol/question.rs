#[derive(Debug, Default)]
pub struct Question {
    pub name: String,
    pub qtype: u16,
    pub qclass: u16,
}

impl Question {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for part in self.name.split('.') {
            bytes.push(part.len() as u8);
            bytes.extend_from_slice(part.as_bytes());
        }
        bytes.push(0);

        bytes.extend_from_slice(&self.qtype.to_be_bytes());
        bytes.extend_from_slice(&self.qclass.to_be_bytes());

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_to_bytes() {
        let question = Question {
            name: "example.com".to_string(),
            qtype: 1,  // A
            qclass: 1, // IN
        };

        let bytes = question.to_bytes();
        let expected = vec![
            7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0, // example.com
            0, 1, // Type A
            0, 1, // Class IN
        ];

        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_question_complex_domain() {
        let question = Question {
            name: "www.google.com".to_string(),
            qtype: 28, // AAAA
            qclass: 1,  // IN
        };

        let bytes = question.to_bytes();
        let expected = vec![
            3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0,
            0, 28, // Type AAAA
            0, 1,   // Class IN
        ];

        assert_eq!(bytes, expected);
    }
}
