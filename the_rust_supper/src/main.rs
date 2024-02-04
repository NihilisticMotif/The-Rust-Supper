#![allow(unused)]
// #[derive(Debug)] // used for debugging

// https://prince.dev/debug-struct
// https://youtu.be/BpPEoZW5IiY?si=goDW5z6qEkPok4pe
// use std::fmt;

//**************************************************************************************************
// FUNCTION NO.1
//**************************************************************************************************
fn match_number01(n: i32) {
    match n {
        // Match a single value
        1 => println!("One!"),
        // Fill in the blank with `|`, DON'T use `..` or `..=`
        2..=5 => println!("match 2 -> 5"),
        // Match an inclusive range
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    }
}
//**************************************************************************************************
// STRUCT NO.2
//**************************************************************************************************
struct Point02 {
    x: i32,
    y: i32,
}

//**************************************************************************************************
// EXAMPLE NO.3
//**************************************************************************************************
enum Message03 {
    Hello { id: i32 },
}

fn match_message03(msg:Message03)->(){
match msg {
    Message03::Hello {
        id:  id@3..=7
    } => println!("Found an id in range [3, 7]: {}", id),
    Message03::Hello { id: newid@(10 | 11 | 12) } => {
        println!("Found an id in another range [10, 12]: {}", newid)
    }
    Message03::Hello { id:i } => println!("Found some other id: {}", i),
}
}

//**************************************************************************************************
// EXAMPLE NO.
//**************************************************************************************************


fn main() {

//**************************************************************************************************
// EXAMPLE NO.1 MATCH INTEGER WITHIN THE RANGE.
//**************************************************************************************************
// match_number01(-11);
// match_number01(2);
// match_number01(5);
// match_number01(6);
// match_number01(8);
// match_number01(88);

//**************************************************************************************************
// EXAMPLE NO.2 @ (AT SIGN)
//**************************************************************************************************
// let p = Point02 { x: 1, y: 10 };
// match p {
//     Point02 { x, y: 0 } => println!("On the x axis at {}", x),
//     // Second arm
//     Point02 { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
//     Point02 { x, y } => println!("On neither axis: ({}, {})", x, y),
// }

//**************************************************************************************************
// EXAMPLE NO.3 IN (CHECK IF THE VALUE WITHIN THE SET) OF MATCH
//**************************************************************************************************
// let msg = Message03::Hello { id: 5 };
// match_message03(msg);
// let msg = Message03::Hello { id: 10 };
// match_message03(msg);
// let msg = Message03::Hello { id: 500 };
// match_message03(msg);

//**************************************************************************************************
// EXAMPLE NO.4 CONDITION IN MATCH SOME
//**************************************************************************************************
// let num = Some(4);
// let split = 5;
// match num {
//     // https://stackoverflow.com/questions/47852269/can-i-use-and-in-match
//     Some(x) if x<split => assert!(x < split),
//     Some(x) => assert!(x >= split),
//     None => (),
// }
// println!("Success!");

//**************************************************************************************************
// EXAMPLE NO.5
//**************************************************************************************************
let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
println("The first element = {}",numbers.0);
println("The first element = {}",numbers.-1);
// match numbers {
//     numbers:first@numbers.0 && => {
//        assert_eq!(first, 2);
//        assert_eq!(last, 2048);
//     }
// }
// println!("Success!");

//**************************************************************************************************
// EXAMPLE NO.
//**************************************************************************************************

//**************************************************************************************************
// EXAMPLE NO.
//**************************************************************************************************

//**************************************************************************************************
// EXAMPLE NO.
//**************************************************************************************************

}

/*
cargo run
*/