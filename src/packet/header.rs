//   0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |                      ID                       |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |                    QDCOUNT                    |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |                    ANCOUNT                    |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |                    NSCOUNT                    |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |                    ARCOUNT                    |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
use crate::packet::flags::Flags;

#[derive(Debug, Clone)]
pub struct Header {
    pub id: u16,
    pub flags: Flags,
    pub qd_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
}

impl Header {
    pub fn new() -> Self {
        Header {
            id: 0,
            flags: Flags::new(),
            qd_count: 0,
            an_count: 0,
            ns_count: 0,
            ar_count: 0,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.id.to_be_bytes());
        bytes.extend(&self.flags.to_u16().to_be_bytes());
        bytes.extend(&self.qd_count.to_be_bytes());
        bytes.extend(&self.an_count.to_be_bytes());
        bytes.extend(&self.ns_count.to_be_bytes());
        bytes.extend(&self.ar_count.to_be_bytes());
        bytes
    }

    pub fn from_bytes(buf: &[u8]) -> Self {
        Header {
            id: u16::from_be_bytes([buf[0], buf[1]]),
            flags: Flags::from_u16(u16::from_be_bytes([buf[2], buf[3]])),
            qd_count: u16::from_be_bytes([buf[4], buf[5]]),
            an_count: u16::from_be_bytes([buf[6], buf[7]]),
            ns_count: u16::from_be_bytes([buf[8], buf[9]]),
            ar_count: u16::from_be_bytes([buf[10], buf[11]]),
        }
    }
}
