#![allow(unused)]

// https://youtu.be/BpPEoZW5IiY?si=goDW5z6qEkPok4pe
fn greetings_box(s: Box<str>) {
    println!("Hello, {}",s);
}
fn greetings_str(s: &str) {
    println!("Hello, {}",s);
}

fn greetings_string(s: String) {
    println!("{}", s)
}
fn main() {
//**************************************************************************************************
// EXAMPLE NO.1, BOX<STR> AND &STR
//**************************************************************************************************
// let name:Box<str>="Jojo".into();
// greetings_box(name);
// // Hello, Jojo
// let name:&str="Jojo";
// greetings_str(name);
// // Hello, Jojo

//**************************************************************************************************
// EXAMPLE NO.2
//**************************************************************************************************
// let s = String::from("I like dogs");
// // Allocate new memory and store the modified string there
// let s1 = s.replace("dogs", "cats");
// assert_eq!(s1, "I like cats");
// println!("{}",s1);

//**************************************************************************************************
// EXAMPLE NO.3 CONCAT STRING
//**************************************************************************************************
// let s1:String = String::from("hello,");
// let s2:String = String::from("world!");
// // let s3 = s1+s2; // invalid
// let s3:String = format!("{}{}",s1,s2); // valid
// let s3:String = s1+&s2;                // valid
// let s1:String = String::from("hello,");
// let s3:String = s1+s2.as_str();        // valid

//**************************************************************************************************
// EXAMPLE NO.4 INSERT STRING::FROM AS THE INPUT OF THE FUNCTION
//**************************************************************************************************
// let s = "hello, world";
// greetings_string(String::from(s))

//**************************************************************************************************
// EXAMPLE NO.5 CONVERT STRING TO &STR
//**************************************************************************************************
// let s = "hello, world".to_string();
// let s1: &str = s.as_str();
// let s1: &str = &s;

//**************************************************************************************************
// EXAMPLE NO.6
//**************************************************************************************************
// // https://stackoverflow.com/questions/56485167/how-to-format-a-byte-into-a-2-digit-hex-string-in-rust
// // https://stackoverflow.com/questions/53353764/how-to-convert-a-very-large-decimal-string-to-hexadecimal
// let b_t='t' as u32;
// println!("b_t (dec) = {}",b_t);
// // b_t (dec) = 116
// let my_string = b_t.to_string();  // `parse()` works with `&str` and `String`!
// let my_int = my_string.parse::<u128>().unwrap();
// let my_hex = format!("{:X}", my_int);
// println!("b_t (hex) = {}", my_hex);     
// // b_t (hex) = 74
// // You can use escapes to write bytes by their hexadecimal values
// // Fill the blank below to show "I'm writing Rust"
// let byte_escape = "I'm writing Ru\x73\x74!";
// println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
// let unicode_codepoint = "\u{211D}";
// let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
// println!("Unicode character {} (U+211D) is called {}",
//             unicode_codepoint, character_name );
// let long_string = "String literals
//                     can span multiple lines.
//                     The linebreak and indentation here \
//                      can be escaped too!";
// println!("{}", long_string);

//**************************************************************************************************
// EXAMPLE NO.7
//**************************************************************************************************
// 3:05:57
// 9 https://practice.rs/compound-types/string.html
}

/*
cargo run
*/
