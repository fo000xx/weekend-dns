use crate::protocol::record::Record;
use crate::reader::Reader;
use rand::RngExt;

pub mod header;
pub mod question;
pub mod record;

use crate::protocol::header::Header;
use crate::protocol::question::Question;

pub const TYPE_A: u16 = 1;
pub const TYPE_NS: u16 = 2;
pub const CLASS_IN: u16 = 1;

#[derive(Debug)]
pub struct DNSPacket {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Record>,
    pub authorities: Vec<Record>,
    pub additionals: Vec<Record>,
}

impl DNSPacket {
    pub fn from_reader(reader: &mut Reader) -> Self {
        let header = Header::from_reader(reader);

        let mut questions = Vec::with_capacity(header.num_questions as usize);
        for _ in 0..header.num_questions {
            questions.push(Question::from_reader(reader));
        }

        let mut answers = Vec::with_capacity(header.num_answers as usize);
        for _ in 0..header.num_answers {
            answers.push(Record::from_reader(reader));
        }

        let mut authorities = Vec::with_capacity(header.num_authorities as usize);
        for _ in 0..header.num_authorities {
            authorities.push(Record::from_reader(reader));
        }

        let mut additionals = Vec::with_capacity(header.num_additonals as usize);
        for _ in 0..header.num_additonals {
            additionals.push(Record::from_reader(reader));
        }

        Self {
            header,
            questions,
            answers,
            authorities,
            additionals,
        }
    }
}

pub fn build_query(domain: String, record_type: u16) -> Vec<u8> {
    let mut rng = rand::rng();
    let id = rng.random_range(0..65535);
    let recursion_desired = 0;
    let header = Header {
        id: id,
        flags: recursion_desired,
        num_questions: 1,
        ..Default::default()
    };
    let question = Question {
        name: domain,
        qtype: record_type,
        qclass: CLASS_IN,
    };

    let mut query_bytes = header.to_bytes().to_vec();
    query_bytes.extend(question.to_bytes());

    query_bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_query() {
        let domain = "example.com".to_string();
        let record_type = TYPE_A;
        let query_bytes = build_query(domain, record_type);

        // Header: 12 bytes
        // Question (example.com): 1+7+1+3+1 = 13 bytes
        // Type: 2 bytes
        // Class: 2 bytes
        // Total expected length: 12 + 13 + 2 + 2 = 29 bytes
        assert_eq!(query_bytes.len(), 29);

        // Check flags: no recursion desired (0x0000)
        assert_eq!(query_bytes[2], 0x00);
        assert_eq!(query_bytes[3], 0x00);

        // Check number of questions: 1
        assert_eq!(query_bytes[4], 0x00);
        assert_eq!(query_bytes[5], 0x01);

        // Check domain serialization (at offset 12)
        assert_eq!(query_bytes[12], 7);
        assert_eq!(&query_bytes[13..20], b"example");
        assert_eq!(query_bytes[20], 3);
        assert_eq!(&query_bytes[21..24], b"com");
        assert_eq!(query_bytes[24], 0);

        // Check record type (at offset 25-26)
        assert_eq!(query_bytes[25], 0);
        assert_eq!(query_bytes[26], TYPE_A as u8);

        // Check class (at offset 27-28)
        assert_eq!(query_bytes[27], 0);
        assert_eq!(query_bytes[28], CLASS_IN as u8);
    }

    #[test]
    fn test_dns_packet_from_reader() {
        let domain = "example.com".to_string();
        let query_bytes = build_query(domain.clone(), TYPE_A);
        let mut reader = Reader::new(&query_bytes);
        let packet = DNSPacket::from_reader(&mut reader);

        assert_eq!(packet.header.num_questions, 1);
        assert_eq!(packet.questions.len(), 1);
        assert_eq!(packet.questions[0].name, domain);
        assert_eq!(packet.answers.len(), 0);
    }
}
