use implementation;

fn main() {
    println!(
        "DNS Resolver client side working model from scratch:
    - For now I have used to delegate the task of resolving to google dns
    - But in future I plan to add my own custom handling of resolving domains\n"
    );
    let msg = implementation::input_url();
    // println!("{:#?}", msg);
    let res = implementation::send_message(msg);
    println!("{:#?}", res);
}
