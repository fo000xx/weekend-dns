#[derive(Debug, Default)]
pub struct Header {
    pub id: u16,
    pub flags: u16,
    pub num_questions: u16,
    pub num_answers: u16,
    pub num_authorities: u16,
    pub num_additonals: u16,
}

impl Header {
    pub fn to_bytes(&self) -> [u8; 12] {
        let mut bytes = [0u8; 12];
        bytes[0..2].copy_from_slice(&self.id.to_be_bytes());
        bytes[2..4].copy_from_slice(&self.flags.to_be_bytes());
        bytes[4..6].copy_from_slice(&self.num_questions.to_be_bytes());
        bytes[6..8].copy_from_slice(&self.num_answers.to_be_bytes());
        bytes[8..10].copy_from_slice(&self.num_authorities.to_be_bytes());
        bytes[10..12].copy_from_slice(&self.num_additonals.to_be_bytes());

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_to_bytes() {
        let header = Header {
            id: 0x1234,
            flags: 0x8180,
            num_questions: 1,
            num_answers: 1,
            num_authorities: 0,
            num_additonals: 0,
        };

        let bytes = header.to_bytes();
        let expected = [
            0x12, 0x34, // ID
            0x81, 0x80, // Flags
            0x00, 0x01, // Questions
            0x00, 0x01, // Answers
            0x00, 0x00, // Authorities
            0x00, 0x00, // Additionals
        ];

        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_header_default() {
        let header = Header::default();
        let bytes = header.to_bytes();
        assert_eq!(bytes, [0u8; 12]);
    }
}
