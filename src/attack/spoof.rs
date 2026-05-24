use crate::packet::rr::ResourceRecord;
use crate::packet::Message;

pub fn simulate_spoof(name: &str, fake_ip: [u8; 4]) -> Message {
    let mut msg = Message::new();
    msg.header.id = 0x1234; // Example ID
    msg.header.flags.qr = true;
    msg.header.an_count = 1;

    let rr = ResourceRecord::new(
        name.to_string(),
        1, // Type A
        1, // Class IN
        3600,
        fake_ip.to_vec(),
    );
    msg.answers.push(rr);
    msg
}
