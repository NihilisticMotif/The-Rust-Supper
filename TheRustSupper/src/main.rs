#![allow(unused)]

// https://youtu.be/BpPEoZW5IiY?si=goDW5z6qEkPok4pe

// use rand::Rng;
// use std::cmp::Ordering;
// use std::fs::File;
// use std::io;
// use std::io::{BufRead, BufReader, ErrorKind, Write};
// use std::ops::{Range, RangeInclusive};
use std::mem::size_of_val;

fn take_ownership(s: String)->String {
    println!("{}", s);
    s
}
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vector u8
    // into_bytes switch the owner of HelloWorld from s to _s
    // as_bytes create new variable, and does not effect the owner ship of HelloWorld.
    let _s = s.as_bytes();
    s
}

fn print_str(s: String) ->String {
    println!("{}",s);
    s
}
fn main() {
//**************************************************************************************************
// EXAMPLE NO.1
//**************************************************************************************************
    // This cause error, because the variable can only have 1 ownership.
    // and String variable is heap variable that only be access by pointer.
    // let x = String::from("Hello world");
    // let y = x;
    // println!("{}, {}",x, y);

    // Solution 1
    // clone copy the heap data.
    // beware that the heap data is memory expensive
    // let x = String::from("Hello world");
    // let y = x.clone();
    // println!("{}, {}",x, y);

    // Solution 2
    // let x = String::from("Hello world");
    // let y = String::from("Hello world");
    // println!("{}, {}",x, y);

    // Solution 3
    // let x = String::from("Hello world");
    // println!("{}",x);

    // Solution 4
    // let x = String::from("Hello world");
    // let y = x;
    // println!("{}",y);

//**************************************************************************************************
// EXAMPLE NO.2
//**************************************************************************************************
    //let s1 = String::from("Hello world");
    //let s2 = take_ownership(s1);
    //println!("{}", s2);

//**************************************************************************************************
// EXAMPLE NO.3
//**************************************************************************************************
    //let s = give_ownership();
    //println!("{}", s);

//**************************************************************************************************
// EXAMPLE NO.4
//**************************************************************************************************
    // let s = String::from("Hello World");
    // let ss = print_str(s);
    // println!("{}", ss);

//**************************************************************************************************
// EXAMPLE NO.5
//**************************************************************************************************
let tupless:(i32,i32,i32,String,(),String)= (255,0,0,"Red".to_string(),()," is my Lucky Color!".to_string());
let x:(i32,i32,(),&str) = (1, 2, (), "hello");
let y = x;
println!("{:?} == {:?}", x, y);
let mut mbti:&str="intp";
mbti="lgbt";
mbti="UFO";
mbti="pseudoscience";
println!("{}",mbti);
/*
The similarity between &str and String
 * Both store string

The different between &str and String
 * String have unknown size
 * &str have fixed size.
*/
}

/*
cargo run
*/
