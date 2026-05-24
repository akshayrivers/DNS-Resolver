pub mod flags;
pub mod header;
pub mod question;
pub mod rr;

use crate::packet::header::Header;
use crate::packet::question::Question;
use crate::packet::rr::ResourceRecord;

#[derive(Debug, Clone)]
pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<ResourceRecord>,
    pub authorities: Vec<ResourceRecord>,
    pub additionals: Vec<ResourceRecord>,
}

impl Message {
    pub fn new() -> Self {
        Message {
            header: Header::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            additionals: Vec::new(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.header.to_bytes();
        for q in &self.questions {
            bytes.extend(q.to_bytes());
        }
        for rr in &self.answers {
            bytes.extend(encode_rr(rr));
        }
        for rr in &self.authorities {
            bytes.extend(encode_rr(rr));
        }
        for rr in &self.additionals {
            bytes.extend(encode_rr(rr));
        }
        bytes
    }

    pub fn from_bytes(buf: &[u8]) -> Self {
        if buf.len() < 12 {
            return Message::new(); // Or handle error
        }
        let header = Header::from_bytes(&buf[0..12]);
        let mut pos = 12;

        let mut questions = Vec::new();
        for _ in 0..header.qd_count {
            if pos >= buf.len() {
                break;
            }
            let (name, next_pos) = parse_qname(buf, pos);
            pos = next_pos;
            if pos + 4 > buf.len() {
                break;
            }
            let qtype = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
            pos += 2;
            let qclass = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
            pos += 2;
            questions.push(Question::new(name, qtype, qclass));
        }

        let mut answers = Vec::new();
        for _ in 0..header.an_count {
            if pos >= buf.len() {
                break;
            }
            let (rr, next_pos) = parse_rr(buf, pos);
            pos = next_pos;
            answers.push(rr);
        }

        let mut authorities = Vec::new();
        for _ in 0..header.ns_count {
            if pos >= buf.len() {
                break;
            }
            let (rr, next_pos) = parse_rr(buf, pos);
            pos = next_pos;
            authorities.push(rr);
        }

        let mut additionals = Vec::new();
        for _ in 0..header.ar_count {
            if pos >= buf.len() {
                break;
            }
            let (rr, next_pos) = parse_rr(buf, pos);
            pos = next_pos;
            additionals.push(rr);
        }

        Message {
            header,
            questions,
            answers,
            authorities,
            additionals,
        }
    }
}

fn encode_rr(rr: &ResourceRecord) -> Vec<u8> {
    let mut bytes = Vec::new();
    for label in rr.name.split('.') {
        bytes.push(label.len() as u8);
        bytes.extend(label.as_bytes());
    }
    bytes.push(0);
    bytes.extend(&rr.rr_type.to_be_bytes());
    bytes.extend(&rr.class.to_be_bytes());
    bytes.extend(&rr.ttl.to_be_bytes());
    bytes.extend(&rr.rd_length.to_be_bytes());
    bytes.extend(&rr.rdata);
    bytes
}

fn parse_rr(buf: &[u8], mut pos: usize) -> (ResourceRecord, usize) {
    let (name, next_pos) = parse_qname(buf, pos);
    pos = next_pos;

    if pos + 10 > buf.len() {
        return (ResourceRecord::new(name, 0, 0, 0, Vec::new()), buf.len());
    }

    let rr_type = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
    pos += 2;
    let class = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
    pos += 2;
    let ttl = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
    pos += 4;
    let rd_length = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
    pos += 2;

    let rdata_end = pos + rd_length as usize;
    if rdata_end > buf.len() {
        return (
            ResourceRecord::new(name, rr_type, class, ttl, Vec::new()),
            buf.len(),
        );
    }
    let rdata = buf[pos..rdata_end].to_vec();
    pos = rdata_end;

    (
        ResourceRecord {
            name,
            rr_type,
            class,
            ttl,
            rd_length,
            rdata,
        },
        pos,
    )
}

fn parse_qname(buf: &[u8], mut pos: usize) -> (String, usize) {
    let mut labels = Vec::new();
    let mut jumped = false;
    let mut original_pos = 0;
    let mut jumps_performed = 0;
    const MAX_JUMPS: u8 = 5; // Prevent infinite loops from malicious pointers

    while pos < buf.len() {
        let byte = buf[pos];

        if byte & 0b11000000 == 0b11000000 {
            if pos + 1 >= buf.len() {
                break;
            }
            let second_byte = buf[pos + 1];
            let pointer_offset = (((byte ^ 0b11000000) as u16) << 8) | (second_byte as u16);

            if !jumped {
                original_pos = pos + 2;
            }

            pos = pointer_offset as usize;
            jumped = true;
            jumps_performed += 1;

            if jumps_performed > MAX_JUMPS {
                break; // Infinite loop protection
            }
            continue;
        }

        if byte == 0 {
            pos += 1;
            break;
        }

        pos += 1;
        let label_length = byte as usize;
        if pos + label_length > buf.len() {
            break; // Malformed label
        }

        let label = &buf[pos..pos + label_length];
        labels.push(String::from_utf8_lossy(label).to_string());
        pos += label_length;
    }

    let qname = labels.join(".");
    if jumped {
        (qname, original_pos)
    } else {
        (qname, pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_qname_basic() {
        let buf = [
            7u8, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0,
        ];
        let (name, pos) = parse_qname(&buf, 0);
        assert_eq!(name, "example.com");
        assert_eq!(pos, 13);
    }

    #[test]
    fn test_parse_qname_pointer() {
        let buf = [
            7u8, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0, 0xC0, 0x00,
        ];
        let (name, pos) = parse_qname(&buf, 13);
        assert_eq!(name, "example.com");
        assert_eq!(pos, 15);
    }

    #[test]
    fn test_malformed_packet_short() {
        let buf = [0u8; 5];
        let msg = Message::from_bytes(&buf);
        assert_eq!(msg.questions.len(), 0);
    }

    #[test]
    fn test_infinite_loop_pointer() {
        // Pointer points to itself
        let buf = [0xC0, 0x00];
        let (name, pos) = parse_qname(&buf, 0);
        assert_eq!(pos, 2); // Should break out
    }
}
