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
        let header = Header::from_bytes(&buf[0..12]);
        let mut pos = 12; //after header

        // Questions = no of questions x [qname,qtype,qclass]
        // now qtype and q class are of fixed size 2 bytes
        // and qname ends with a zero-length byte (0) 7example3com0 so that is how we will parse Questions

        let mut questions = Vec::new();
        for _ in 0..header.qd_count {
            let (name, next_pos) = parse_qname(buf, pos);
            pos = next_pos;
            let qtype = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
            pos += 2;
            let qclass = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
            pos += 2;
            questions.push(Question::new(name, qtype, qclass));
        }

        // Answers, Authority, Additional - Are all resource records x no.of items(from header)
        // type=2 class=2 TTL=4 rd_length=2 and rd_data encompasses rd length
        // the name hah! is saved often using pointer compression. And what is pointer compression you ask?
        let mut answers = Vec::new();
        for _ in 0..header.an_count {
            let (rr, next_pos) = parse_rr(buf, pos);
            pos = next_pos;
            answers.push(rr);
        }

        let mut authorities = Vec::new();
        for _ in 0..header.ns_count {
            let (rr, next_pos) = parse_rr(buf, pos);
            pos = next_pos;
            authorities.push(rr);
        }

        let mut additionals = Vec::new();
        for _ in 0..header.ar_count {
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
    // Simplified encoding for now (no compression on write)
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
// Answers, Authority, Additional - Are all resource records x no.of items(from header)
// type=2 class=2 TTL=4 rd_length=2 and rd_data encompasses rd length
// the name hah! is saved often using pointer compression. And what is pointer compression you ask?
fn parse_rr(buf: &[u8], mut pos: usize) -> (ResourceRecord, usize) {
    let (name, next_pos) = parse_qname(buf, pos);
    pos = next_pos;

    let rr_type = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
    pos += 2;
    let class = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
    pos += 2;
    let ttl = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
    pos += 4;
    let rd_length = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
    pos += 2;
    let rdata = buf[pos..pos + rd_length as usize].to_vec();
    pos += rd_length as usize;

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

// okay this is made to handle name parsing I. Qusetion we just see if byte is 00 for eg: 03 'w' 'w' 'w' 07 'e' 'x' 'a' 'm' 'p' 'l' 'e' 03 'c' 'o' 'm' 00
// II. okay so pointer compression is just that we don't waste bytes we just add the pointer the names where it has appeared before in the buffer
// The first two bits of a length byte set to 11 (binary) or 0xC0 (hex) indicate a pointer
// The next 14 bits represent the offset in the message where the rest of the domain name can be found.
//         Example:
// Suppose somewhere in the DNS message, at position 20, we already had:

// 07 'e' 'x' 'a' 'm' 'p' 'l' 'e' 03 'c' 'o' 'm' 00
// Later, instead of repeating "example.com", the message can use a pointer like:

// C0 14
// C0 = 11000000 binary → pointer marker
// 14 (hex) = 20 decimal → offset to position 20 where "example.com" starts
fn parse_qname(buf: &[u8], mut pos: usize) -> (String, usize) {
    let mut labels = Vec::new();
    let mut jumped = false;
    let mut original_pos = 0;

    loop {
        let byte = buf[pos];
        // Checking if the first two bits are 1 1 (pointer)
        if byte & 0b11000000 == 0b11000000 {
            let second_byte = buf[pos + 1];
            // this part was hell

            // “Just stick the two bytes together — that’s the pointer, right?”
            // But what we really need is:

            // “Use the last 6 bits of the first byte and all 8 bits of the second byte to build a 14-bit number.

            // lets take another example: a very simple and plain analogy:
            // If you have two digits: 4 and 2, and you want to make 42, you multiply the first by 10 and add the second.

            // In binary:
            // If you have two bytes: 0x01 and 0x0C, and want to make 0x010C, you shift the first by 8 and add the second.

            // now we extract the pointer
            // We Remove the two high bits 11000000 because they just show the that the next 14 bits is a pointer
            let pointer_offset = (((byte ^ 0b11000000) as u16) << 8) | (second_byte as u16);
            // Save current position only the first time we jump
            if !jumped {
                original_pos = pos + 2; // like from where do we continue after this
            }
            // We Add(OR) the two parts into the full 14-bit offset which is actually u16
            pos = pointer_offset as usize;
            jumped = true;
            continue;
        }
        // If byte is 0, end of the QNAME hex(00)
        if byte == 0 {
            pos += 1;
            break;
        }

        pos += 1;
        let label_length = byte as usize;
        let end = pos + label_length;
        let label = &buf[pos..end];
        labels.push(String::from_utf8_lossy(label).to_string());
        pos += label_length;
    }

    let qname = labels.join(".");
    // Return the position we stopped at
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
        // example.com encoded as [7]example[3]com[0]
        let buf = [
            7u8, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0,
        ];
        let (name, pos) = parse_qname(&buf, 0);
        assert_eq!(name, "example.com");
        assert_eq!(pos, 13);
    }

    #[test]
    fn test_parse_qname_pointer() {
        // Buffer layout:
        // 0..12: 7 'e' 'x' 'a' 'm' 'p' 'l' 'e' 3 'c' 'o' 'm' 0   (example.com)
        // 12..16: some filler bytes (x, a, c, ... )
        // 16: pointer 0xC000 (11000000 00000000) pointing to offset 0, i.e. "example.com"
        // After pointer comes some bytes representing "reachhere" (just filler)
        let buf = [
            7u8, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0, 0xC0, 0x00,
        ];
        let (name, pos) = parse_qname(&buf, 13);
        assert_eq!(name, "example.com");
        assert_eq!(pos, 15);
    }
}
