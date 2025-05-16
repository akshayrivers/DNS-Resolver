use std::io;
use std::net::UdpSocket;
use std::time::Duration;
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
            qtype: 28, // A record  we are hardcoding it 1-Ipv4 , 2-NS ,5- CName,15-MX, 28-Ipv6
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

    pub fn from_bytes(buf: &[u8]) -> Self {
        // Now we know that the header section is of 12 bytes from the start
        // 0-11 now we get the data for the next bytes from this like how many questions[qname,qtype,qclass], [RR]answers, authority , additional info

        // Parse header (first 12 bytes)
        let header = DnsHeader {
            identification: u16::from_be_bytes([buf[0], buf[1]]),
            flags: u16::from_be_bytes([buf[2], buf[3]]),
            no_of_questions: u16::from_be_bytes([buf[4], buf[5]]),
            no_of_answers_rr: u16::from_be_bytes([buf[6], buf[7]]),
            no_of_authority_rr: u16::from_be_bytes([buf[8], buf[9]]),
            no_of_additional_rr: u16::from_be_bytes([buf[10], buf[11]]),
        };

        // Questions = no of questions x [qname,qtype,qclass]
        // now qtype and q class are of fixed size 2 bytes
        // and qname ends with a zero-length byte (0) 7example3com0 so that is how we will parse Questions

        let mut pos = 12; // after header
        let mut questions = Vec::new();

        for _ in 0..header.no_of_questions {
            let (qname, next_pos) = parse_qname(buf, pos);
            pos = next_pos;
            let qtype = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
            pos += 2;
            let qclass = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
            pos += 2;
            questions.push(DnsQuestion {
                qname,
                qtype,
                qclass,
            });
        }

        // Answers, Authority, Additional - Are all resource records x no.of items(from header)
        // type=2 class=2 TTL=4 rd_length=2 and rd_data encompasses rd length
        // the name hah! is saved often using pointer compression. And what is pointer compression you ask?

        fn parse_rr(buf: &[u8], mut pos: usize) -> (ResourceRecord, usize) {
            let (name, new_pos) = parse_qname(buf, pos);
            pos = new_pos;

            let rr_type = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
            pos += 2;

            let class = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
            pos += 2;

            let ttl = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
            pos += 4;

            let rdlength = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
            pos += 2;

            let rdata = buf[pos..pos + rdlength as usize].to_vec();
            pos += rdlength as usize;

            (
                ResourceRecord {
                    name,
                    rr_type,
                    class,
                    ttl,
                    rdlength,
                    rdata,
                },
                pos,
            )
        }

        let mut answers = Vec::new();
        for _ in 0..header.no_of_answers_rr {
            let (rr, new_pos) = parse_rr(buf, pos);
            pos = new_pos;
            answers.push(rr);
        }

        let mut authority = Vec::new();
        for _ in 0..header.no_of_authority_rr {
            let (rr, new_pos) = parse_rr(buf, pos);
            pos = new_pos;
            authority.push(rr);
        }

        let mut additional = Vec::new();
        for _ in 0..header.no_of_additional_rr {
            let (rr, new_pos) = parse_rr(buf, pos);
            pos = new_pos;
            additional.push(rr);
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
                    // this part is fucking hell

                    // “Just stick the two bytes together — that’s the pointer, right?”
                    // But what we really need is:

                    // “Use the last 6 bits of the first byte and all 8 bits of the second byte to build a 14-bit number.

                    // lets take another example: a very simple and plain analogy:
                    // If you have two digits: 4 and 2, and you want to make 42, you multiply the first by 10 and add the second.

                    // In binary:
                    // If you have two bytes: 0x01 and 0x0C, and want to make 0x010C, you shift the first by 8 and add the second.

                    // now we extract the pointer
                    // We Remove the two high bits 11000000 because they just show the that the next 14 bits is a pointer
                    let upper_pointer_bits = byte ^ 0b11000000;

                    //  shift left by 8 bits - well the first 6 bits of the pointer contribution
                    // keep in mind that the pointer is still 2 bytes that is why we cast it left by 8 bits
                    let upper_offset = (upper_pointer_bits as u16) << 8;

                    let lower_offset = second_byte as u16;

                    // We Add(OR) the two parts into the full 14-bit offset which is actually u16
                    let pointer_offset = upper_offset | lower_offset;

                    // Save current position only the first time we jump
                    if !jumped {
                        original_pos = pos + 2; // like from where do we continue after this
                    }

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
                pos += byte as usize;
            }

            let qname = labels.join(".");

            // Return the position we stopped at
            if jumped {
                (qname, original_pos)
            } else {
                (qname, pos)
            }
        }

        DnsMessage {
            header,
            question: questions.into_iter().next().unwrap_or(DnsQuestion {
                qname: "".to_string(),
                qtype: 0,
                qclass: 0,
            }),
            answers,
            authority,
            additional,
        }
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

pub fn send_message(msg: DnsMessage) -> DnsMessage {
    // 1. creating a DNS message and then turning it into bytes and then send it to the 8.8.8.8 for now we are not handling the complexities ourself
    let server = "8.8.8.8:53"; // Google DNS
    let socket = UdpSocket::bind("0.0.0.0:0").expect("could not bind to address");

    // Optional: set a timeout
    socket
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();

    let message_bytes = msg.to_bytes();

    // Send to DNS server
    socket
        .send_to(&message_bytes, server)
        .expect("failed to send DNS query");

    // Receive response
    let mut buf = [0u8; 512]; // Max size for a DNS response is 512 bytes
    let (size, _) = socket
        .recv_from(&mut buf)
        .expect("did not receive a response");

    // okay so now we have our bytes with us from in the buf so we try to parse it into the message again
    let res = DnsMessage::from_bytes(&buf[..size]);
    res
}
