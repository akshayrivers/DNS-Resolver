#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flags {
    pub qr: bool,   // Query/Response (false for query, true for response)
    pub opcode: u8, // Kind of query (4 bits)
    pub aa: bool,   // Authoritative Answer
    pub tc: bool,   // Truncation
    pub rd: bool,   // Recursion Desired
    pub ra: bool,   // Recursion Available
    pub z: u8,      // Reserved (3 bits)
    pub rcode: u8,  // Response code (4 bits)
}

impl Flags {
    pub fn new() -> Self {
        Flags {
            qr: false,
            opcode: 0,
            aa: false,
            tc: false,
            rd: false,
            ra: false,
            z: 0,
            rcode: 0,
        }
    }

    pub fn to_u16(&self) -> u16 {
        let mut flags = 0u16;
        if self.qr {
            flags |= 1 << 15;
        }
        flags |= (self.opcode as u16 & 0x0F) << 11;
        if self.aa {
            flags |= 1 << 10;
        }
        if self.tc {
            flags |= 1 << 9;
        }
        if self.rd {
            flags |= 1 << 8;
        }
        if self.ra {
            flags |= 1 << 7;
        }
        flags |= (self.z as u16 & 0x07) << 4;
        flags |= self.rcode as u16 & 0x0F;
        flags
    }

    pub fn from_u16(flags: u16) -> Self {
        Flags {
            qr: (flags >> 15) & 1 != 0,
            opcode: ((flags >> 11) & 0x0F) as u8,
            aa: (flags >> 10) & 1 != 0,
            tc: (flags >> 9) & 1 != 0,
            rd: (flags >> 8) & 1 != 0,
            ra: (flags >> 7) & 1 != 0,
            z: ((flags >> 4) & 0x07) as u8,
            rcode: (flags & 0x0F) as u8,
        }
    }
}
