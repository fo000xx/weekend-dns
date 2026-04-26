pub struct Reader<'a> {
    pub buf: &'a [u8],
    pub pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn read(&mut self, len: usize) -> &[u8] {
        let res = &self.buf[self.pos..self.pos + len];
        self.pos += len;
        res
    }

    pub fn read_u16(&mut self) -> u16 {
        u16::from_be_bytes(self.read(2).try_into().unwrap())
    }

    pub fn read_u32(&mut self) -> u32 {
        u32::from_be_bytes(self.read(4).try_into().unwrap())
    }

    pub fn decode_name(&mut self) -> String {
        let mut parts = Vec::new();
        let mut current_pos = self.pos;
        let mut jumped = false;
        let mut next_pos = 0;

        loop {
            let length = self.buf[current_pos];

            // Check if it's a pointer (first two bits are 11)
            if (length & 0xC0) == 0xC0 {
                // If this is our first jump, we need to remember where to
                // return to so the reader can continue correctly.
                if !jumped {
                    next_pos = current_pos + 2;
                }

                // Calculate pointer offset (14 bits)
                let b2 = self.buf[current_pos + 1] as usize;
                let offset = (((length & 0x3F) as usize) << 8) | b2;

                current_pos = offset;
                jumped = true;
            } else {
                current_pos += 1;
                if length == 0 {
                    break;
                }

                let end = current_pos + length as usize;
                let label = &self.buf[current_pos..end];
                parts.push(String::from_utf8_lossy(label).to_string());
                current_pos = end;
            }
        }

        // If we jumped, we return to the byte immediately after the first pointer.
        // Otherwise, we stay at the position after the null (0) terminator.
        if jumped {
            self.pos = next_pos;
        } else {
            self.pos = current_pos;
        }

        parts.join(".")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_name_simple() {
        let buf = b"\x03www\x07example\x03com\x00";
        let mut reader = Reader::new(buf);
        assert_eq!(reader.decode_name(), "www.example.com");
        assert_eq!(reader.pos, buf.len());
    }

    #[test]
    fn test_decode_name_compressed() {
        // Header (12 bytes) + Name "google.com" (12 bytes) + Pointer to offset 12
        let mut buf = vec![0u8; 12];
        buf.extend_from_slice(b"\x06google\x03com\x00");
        buf.extend_from_slice(b"\x03www\xc0\x0c"); // www. + pointer to google.com (offset 12)

        let mut reader = Reader::new(&buf);
        reader.pos = 24; // Start at "www..."
        assert_eq!(reader.decode_name(), "www.google.com");
        assert_eq!(reader.pos, 24 + 6); // Should be exactly after the pointer (\x03www\xc0\x0c)
    }
}
