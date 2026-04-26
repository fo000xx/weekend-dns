use crate::protocol::TYPE_NS;
use crate::reader::Reader;

#[derive(Debug, PartialEq)]
pub struct Record {
    pub name: String,
    pub rtype: u16,
    pub rclass: u16,
    pub ttl: u32,
    pub data: Vec<u8>,
}

impl Record {
    pub fn from_reader(reader: &mut Reader) -> Self {
        let name = reader.decode_name();
        let rtype = reader.read_u16();
        let rclass = reader.read_u16();
        let ttl = reader.read_u32();
        let data_len = reader.read_u16();

        let data = if rtype == TYPE_NS {
            reader.decode_name().into_bytes()
        } else {
            reader.read(data_len as usize).to_vec()
        };

        Self {
            name,
            rtype,
            rclass,
            ttl,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::TYPE_A;

    #[test]
    fn test_record_from_reader() {
        let bytes = vec![
            3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm',
            0, 0, 1, // Type A
            0, 1, // Class IN
            0, 0, 0, 60, // TTL 60
            0, 4, // Data len 4
            93, 184, 216, 34, // IP 93.184.216.34
        ];

        let mut reader = Reader::new(&bytes);
        let record = Record::from_reader(&mut reader);

        assert_eq!(record.name, "www.example.com");
        assert_eq!(record.rtype, TYPE_A);
        assert_eq!(record.rclass, 1);
        assert_eq!(record.ttl, 60);
        assert_eq!(record.data, vec![93, 184, 216, 34]);
        assert_eq!(reader.pos, bytes.len());
    }

    #[test]
    fn test_ns_record_from_reader() {
        let bytes = vec![
            7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0,
            0, 2, // Type NS
            0, 1, // Class IN
            0, 2, 163, 0, // TTL 172800 (0x0002A300)
            0, 4, // Data len (not used by our parser for NS, but present on wire)
            3, b'n', b's', b'1', 0xc0, 0x00, // ns1. + pointer to example.com at offset 0
        ];

        let mut reader = Reader::new(&bytes);
        let record = Record::from_reader(&mut reader);

        assert_eq!(record.name, "example.com");
        assert_eq!(record.rtype, TYPE_NS);
        assert_eq!(String::from_utf8_lossy(&record.data), "ns1.example.com");
    }
}
