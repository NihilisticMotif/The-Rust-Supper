#![allow(unused)]
// https://stackoverflow.com/questions/37410672/expected-type-parameter-found-u8-but-the-type-parameter-is-u8
// use std::fmt::{self, Debug};
// https://prince.dev/debug-struct
// https://youtu.be/BpPEoZW5IiY?si=goDW5z6qEkPok4pe
// use std::fmt;
//**************************************************************************************************
// EXAMPLE NO.1 METHOD
//**************************************************************************************************
struct A;          // Concrete type `A`.
struct S(A);       // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn main() {
//**************************************************************************************************
// EXAMPLE NO.1 METHOD
//**************************************************************************************************
// Using the non-generic functions
reg_fn(S(A));          // Concrete type.
gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
gen_spec_i32(SGen(4)); // Implicitly specified type parameter `i32`.
// Explicitly specified type parameter `char` to `generic()`.
generic::<char>(SGen('a'));
// Implicitly specified type parameter `char` to `generic()`.
generic(SGen('a'));
println!("Success!");

//**************************************************************************************************
// EXAMPLE NO.1 METHOD
//**************************************************************************************************


}

/*
cargo run
*/