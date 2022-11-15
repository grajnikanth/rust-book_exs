use std::error::Error;


enum CounterInstruction {
    Increment, // unsigned byte
    Decrement, // unsigned byte
}

impl CounterInstruction {
    fn deserialize(buf: &mut &[u8]) -> Self {

        let res = buf[0];
        res
    }
}

fn main() {
    let data = [0u8];
    let instance_1 = CounterInstruction::deserialize(&mut data);
    println!("instance_1 converted to enum is {:?}", instance_1);

}