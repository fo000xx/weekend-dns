use crate::reader::Reader;

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

    pub fn from_reader(reader: &mut Reader) -> Self {
        Self {
            id: reader.read_u16(),
            flags: reader.read_u16(),
            num_questions: reader.read_u16(),
            num_answers: reader.read_u16(),
            num_authorities: reader.read_u16(),
            num_additonals: reader.read_u16(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_from_reader() {
        let bytes = [
            0x12, 0x34, // ID
            0x81, 0x80, // Flags
            0x00, 0x01, // Questions
            0x00, 0x01, // Answers
            0x00, 0x00, // Authorities
            0x00, 0x00, // Additionals
        ];
        let mut reader = Reader::new(&bytes);
        let header = Header::from_reader(&mut reader);

        assert_eq!(header.id, 0x1234);
        assert_eq!(header.flags, 0x8180);
        assert_eq!(header.num_questions, 1);
        assert_eq!(header.num_answers, 1);
        assert_eq!(reader.pos, 12);
    }

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
