#![allow(unused)]

// Homeworks
// 1. Read this: แ
// 2. code function with generic
// 3. code multitypes struct with <T,U> generics
// 4. code enum with generics
// 5. code method that only works with specific type (e.g. f64, i32 etc.)
// 6. Copy this code and paste it in SimpleStruct/Generics.txt, then clear main.rs
// 6. Solve this: https://practice.course.rs/generics-traits/generics.html
// 7. Watch this: https://youtu.be/BpPEoZW5IiY?si=ny9xkG2F8CqQemql (5:33:00)

//**************************************************************************************************
// FUNCTION THAT TAKES GENERICS INPUT
//**************************************************************************************************
use std::fmt;
use std::fmt::Display;
fn printing<T: Display>(arg: T) {
    // https://www.reddit.com/r/learnrust/comments/hrv6az/println_in_another_function_with_generics/
    println!("{}", arg)
}

fn printloop<T: Display, const COUNT: usize>(arg: [T; COUNT]) {
    for i in arg {
        println!("{}", i);
    }
}

fn naming<T: Display>(arg: T, sirname: String) -> String {
    format!("{}{}", arg, sirname)
}

//**************************************************************************************************
// GENERIC ARRAY
//**************************************************************************************************
#[derive(Debug)]
// https://stackoverflow.com/questions/28136739/is-it-possible-to-control-the-size-of-an-array-using-the-type-parameter-of-a-gen
// https://stackoverflow.com/questions/28136739/is-it-possible-to-control-the-size-of-an-array-using-the-type-parameter-of-a-gen
// https://www.reddit.com/r/learnrust/comments/hrv6az/println_in_another_function_with_generics/
struct Array<T: Display, const COUNT: usize> {
    data: [T; COUNT],
}

impl<T: Display, const COUNT: usize> Array<T, COUNT> {
    fn printloop(&self) -> () {
        for i in &self.data {
            println!("{}", i);
        }
    }

    fn longname(&self) -> String {
        let mut name = "".to_string();
        for i in &self.data {
            name.push_str(&format!("{}", i));
        }
        name
    }

    fn joining(&self, space: String) -> String {
        let mut name = "".to_string();
        for i in &self.data {
            name.push_str(&format!("{}{}", i, space));
        }
        name
    }

    fn lengths(&self) -> u32 {
        self.data.len() as u32
    }
}

#[derive(Debug)]
struct VectorS<T, const COUNT: usize> {
    data: [T; COUNT],
}

impl<const COUNT: usize> VectorS<i32, COUNT> {
    // https://www.becomebetterprogrammer.com/rust-fix-doesnt-implement-std-fmt-display/
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ", ")
    }
    fn sigma(&self) -> i32 {
        let mut y = 0;
        for i in self.data {
            y = y + i;
        }
        y
    }
    fn means(&self) -> f64 {
        let mut y = 0;
        for i in self.data {
            y = y + i;
        }
        y as f64 / self.data.len() as f64
    }
    fn std(&self) -> f64 {
        let mut y = 0 as f64;
        let mean = self.means();
        let n = self.data.len();
        for i in self.data {
            y = y + (i as f64 - mean).powf(2.0);
        }
        y / mean
    }
    fn product(&self) -> i32 {
        let mut y = 0;
        for i in self.data {
            y = y + i;
        }
        y
    }

    fn dotproduct(&self) {
        //
    }
}

//**************************************************************************************************
// STRUCT WITH 2 GENERIC TYPES (T,U)
//**************************************************************************************************
#[derive(Debug)]
struct Dita<T, U> {
    data1: T,
    data2: U,
}

impl Dita<String, u32> {
    fn newnum(s: u32) -> Self {
        Dita {
            data1: format!("DitaNo{}", s).to_string(),
            data2: s,
        }
    }
    fn printing(&self) -> () {
        println!("{} is {} year old.", self.data1, self.data2);
    }
}

impl Dita<i32, i32> {
    fn summ(&self) -> i32 {
        self.data1 + self.data2
    }
}

fn main() {
    printing(1);
    printing('a');
}

/*
cargo run
*/
