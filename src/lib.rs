use std::io;
#[derive(Debug)]
pub struct DnsHeader {
    // header section - 12 bytes
    pub identification: u16,
    pub flags: u16, // for now we are planning on hardcoding it because we are just using recursion
    pub no_of_questions: u16,
    pub no_of_answers_rr: u16,
    pub no_of_authority_rr: u16,
    pub no_of_additional_rr: u16,
}
#[derive(Debug)]
pub struct DnsQuestion {
    //Name and type feilds for a query
    pub qname: String, // example.com
    pub qtype: u16,    // A = 1
    pub qclass: u16,   // IN = 1
}
#[derive(Debug)]
pub struct ResourceRecord {
    pub name: String,
    pub rr_type: u16, // A = 1, NS = 2, etc.
    pub class: u16,   // Usually IN (1)
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>, // Parsed separately depending on type
}
#[derive(Debug)]
pub struct DnsMessage {
    pub header: DnsHeader,
    pub question: DnsQuestion,
    pub answers: Vec<ResourceRecord>,    // RRs in response to query
    pub authority: Vec<ResourceRecord>,  //Records for authoritative servers
    pub additional: Vec<ResourceRecord>, //Additional helpful info
}

impl DnsMessage {
    pub fn new(url: String) -> Self {
        let header = DnsHeader {
            identification: 0x1234, // random ID hardcoded for now
            flags: 0x0100,          // we will send the recursion request
            no_of_questions: 1,
            no_of_answers_rr: 0,
            no_of_authority_rr: 0,
            no_of_additional_rr: 0,
        };

        let question = DnsQuestion {
            qname: url,
            qtype: 1,  // A record  we are hardcoding it
            qclass: 1, // IN (Internet)
        };

        DnsMessage {
            header,
            question,
            // the next section we will get a response back
            answers: Vec::new(),
            authority: Vec::new(),
            additional: Vec::new(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // HEADER SECTION
        bytes.extend(&self.header.identification.to_be_bytes()); // 2 bytes
        bytes.extend(&self.header.flags.to_be_bytes()); // 2 bytes
        bytes.extend(&self.header.no_of_questions.to_be_bytes()); // 2 bytes
        bytes.extend(&self.header.no_of_answers_rr.to_be_bytes()); // 2 bytes
        bytes.extend(&self.header.no_of_authority_rr.to_be_bytes()); // 2 bytes
        bytes.extend(&self.header.no_of_additional_rr.to_be_bytes()); // 2 bytes

        // QUESTION SECTION
        // QNAME — example.com becomes [7]example[3]com[0]
        for label in self.question.qname.split('.') {
            bytes.push(label.len() as u8); // length byte
            bytes.extend(label.as_bytes()); // label bytes
        }
        bytes.push(0); // end of QNAME

        // QTYPE (2 bytes)
        bytes.extend(&self.question.qtype.to_be_bytes());

        // QCLASS (2 bytes)
        bytes.extend(&self.question.qclass.to_be_bytes());

        bytes
    }
}

pub fn input_url() -> DnsMessage {
    let mut input = String::new();
    println!("Input the domain name you want to resolve: ");
    io::stdin().read_line(&mut input).unwrap();
    let url = input.trim();
    let msg = DnsMessage::new(url.to_owned());
    return msg;
}
