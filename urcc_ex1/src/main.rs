const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);
    // shadowing variable to avoid mutability issues, else 'missiles' would have to be defined as
    // mutable, 'let mut missiles'.
    //let missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
}
