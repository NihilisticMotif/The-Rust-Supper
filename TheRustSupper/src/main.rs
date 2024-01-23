#![allow(unused)]

// https://youtu.be/BpPEoZW5IiY?si=goDW5z6qEkPok4pe
// #[derive(Debug)] // used for debugging
enum Number {
    Zero,
    One,
    Two,
}
enum Number1 {
    Zero = 0,
    One,
    Two,
}
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
//**************************************************************************************************
// EXAMPLE NO.1 INITIALIZE ENUM
//**************************************************************************************************
// assert_eq!(Number::One as i32, Number1::One as i32);
// assert_eq!(Number1::One as i32, Number2::One as i32);

//**************************************************************************************************
// EXAMPLE NO.2 EACH ENUM CAN HOLD ITS OWN DATA.
//**************************************************************************************************
// let msg1 = Message::Move{x:1_i32,y:2_i32}; // Instantiating with x = 1, y = 2 
// let msg2 = Message::Write("hello, world!".to_string()); // Instantiating with "hello, world!"

//**************************************************************************************************
// EXAMPLE NO.1 
//**************************************************************************************************
// let msg = Message::Move{x: 1, y: 2};
// if let Message::Move{ x: i32, y: i32 } = msg {
//     assert_eq!(a, b);
// } else {
//     panic!("NEVER LET THIS RUN！");
// }
// println!("Success!");

//**************************************************************************************************
// EXAMPLE NO.1 
//**************************************************************************************************

//**************************************************************************************************
// EXAMPLE NO.1 
//**************************************************************************************************

// 04:02:52 Enums

}

/*
cargo run
*/
