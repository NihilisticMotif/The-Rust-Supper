#![allow(unused)]

// Homeworks
// 1. Read this: 
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
// DISPLAY ARRAY
//**************************************************************************************************
#[derive(Debug)]
// https://stackoverflow.com/questions/28136739/is-it-possible-to-control-the-size-of-an-array-using-the-type-parameter-of-a-gen
// https://stackoverflow.com/questions/28136739/is-it-possible-to-control-the-size-of-an-array-using-the-type-parameter-of-a-gen
// https://www.reddit.com/r/learnrust/comments/hrv6az/println_in_another_function_with_generics/
struct Array<T: Display, const COUNT: usize> {
    data: [T; COUNT],
}

impl Array<u32,10>{
    fn new(n:u32)->Self{
        const N:usize=10;
        let y:[u32;N]=[n;N];
        Array{data:y}
    }
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

//**************************************************************************************************
// F64 ARRAY
//**************************************************************************************************

#[derive(Debug)]
struct VectorS<T, const COUNT: usize> {
    data: [T; COUNT],
}

impl <const COUNT: usize> VectorS <f64,COUNT> {
    // https://www.becomebetterprogrammer.com/rust-fix-doesnt-implement-std-fmt-display/
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ", ")
    }

    fn sigma(&self) -> f64 {
        let mut y = 0.0;
        for i in self.data {
            y = y + i;
        }
        y
    }

    fn means(&self) -> f64 {
        let mut y = 0.0;
        for i in self.data {
            y = y + i;
        }
        y as f64 / self.data.len() as f64
    }

    fn std(&self) -> f64 {
        let mut y = 0.0;
        let mean = self.means();
        for i in self.data {
            y = y + (i as f64 - mean).powf(2.0);
        }
        y / mean
    }

    fn product(&self) -> f64 {
        let mut y = 0.0;
        for i in self.data {
            y = y * i;
        }
        y
    }

    fn dotproduct(&self,v:Self) ->f64{
        let mut y = 0.0;
        let mut count=0;
        for i in self.data {
            y = y + i * v.data[count] ;
            count+=1;
        }
        y
    }
}

//**************************************************************************************************
// STRUCT WITH MULTIPLE GENERIC TYPES (T,U)
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
    fn product(&self) -> i32 {
        self.data1 * self.data2
    }
    fn subb(&self) -> i32 {
        self.data1 - self.data2
    }
}

//**************************************************************************************************
// ENUM WITH GENERIC 2 TYPES
//**************************************************************************************************
enum DF<X,Y:Display,const N:usize>{
    TrainX([X; N]),
    TrainY(Y),
    TestXX([X; N]),
    TestYY(Y)
}


fn data_shape<X,Y:Display,const N:usize>(df:DF<X,Y,N>){
    match df{
        DF::TrainX(x)=>{println!("TrainX = {}",x.len())},
        DF::TrainY(x)=>{println!("TrainY = {}",x)},
        DF::TestXX(x)=>{println!("TestXX = {}",x.len())},
        DF::TestYY(x)=>{println!("TestYY = {}",x)},
    }
}

fn predict<const N:usize>(df:DF<f64,f64,N>,weight:[f64; N],bias:f64)->f64{
    match df{
        DF::TrainX(x)=>{
            dotproduct_f64(x,weight)+bias
        },
        DF::TestXX(x)=>{
            dotproduct_f64(x,weight)+bias
        },
        _=>{
            println!("Input is invalid");
            0.0
        }
    }
}

fn mean_square_error_f64<const N:usize>(df:DF<f64,f64,N>,weight:[f64; N],bias:f64,y:f64)->f64{
    let prediction = predict(df,weight,bias);
    (prediction-y).powf(2.0)
}

fn dotproduct_f64<const N:usize>(v:[f64; N],u:[f64; N])->f64{
    let mut count=0;
    let mut y=0.0;
    for i in v{
        y += i*u[count];
        count+=1;
    }
    y
}

//**************************************************************************************************
// RAIT
//**************************************************************************************************
// https://doc.rust-lang.org/book/ch10-02-traits.html
trait SayMyName{
    fn summarize_author(&self) -> String{
        String::from("Unknown")
    }
    fn say_my_name(&self) -> String{
        format!("{} (Unknown Type)",self.summarize_author())
    }
}

trait BirthDay{
    fn birth_day(&self) -> String;
}

enum Occupation{
    Programmer(String),
    Engineer(String),
    Scientist(String),
    Doctor(String),
    Artist(String),
    Writer(String),
    Teacher(String),
    Politician(String),
    Lawyer(String),
    Farmer(String)
}

enum TierList{
    S,
    A,
    B,
    C,
    D,
    F
}

struct Person{
    name:String,
    age:u32,
    occupation:Occupation,
    tierlist:TierList
}

impl SayMyName for Person{
    fn say_my_name(&self)->String{
        format!("{} (Person)",self.name.to_string())
    }
}

struct CEO<const N:usize>{
    name:String,
    age:u32,
    field:String,
    workers:[Person;N]
}

impl <const N:usize>SayMyName for CEO<N>{
    fn say_my_name(&self)->String{
        format!("{} (CEO)",self.name.to_string())
    }
}

// https://users.rust-lang.org/t/how-to-exclude-a-type-from-generic-trait-implementation/26156/5
impl <T:Display,const COUNT: usize> SayMyName for Array<T, COUNT> {}
impl <const COUNT: usize> SayMyName for [Array<&str, COUNT>] {
    fn summarize_author(&self) -> String{
        (*self)[0].data[0].to_string()
    }
}

// trait PrintElement{
//     fn print_element(&self,nums:u32) ->(){
//         println!("{}",self.data[nums]);
//     }
// }
// impl <T: Display, const COUNT: usize> PrintElement for Array<T, COUNT> {}

//**************************************************************************************************
// MAIN
//**************************************************************************************************

fn main() {
println!("//**************************************************************************************************");
println!("// FUNCTION THAT TAKES GENERICS INPUT");
println!("//**************************************************************************************************");
printing(3.14159);
printing(2.71828);
printing('A');
printing("Hello World");
printloop(["Hello World","ChatGPT","Alphafold","Alphazero","StockFish"]);
println!("{}",naming(3.14159, "3B1B".to_string()));
println!("{}",naming(2.71828, "Euler".to_string()));
println!("{}",naming(3, "Pythagorus".to_string()));

println!("//**************************************************************************************************");
println!("// DISPLAY ARRAY");
println!("//**************************************************************************************************");
let avatar_friends=Array{data:["Aang","Katara","Zuko","Sokka","Toph","Suki","Bumi","Appa","Kuzon","Azula","Tylee","Mia"]};
printloop(avatar_friends.data);
avatar_friends.printloop();
println!("avatar_friends.longname() = {}",avatar_friends.longname());
println!("avatar_friends = {}",avatar_friends.joining("_|_".to_string()));
println!("There are {} avatar friends",avatar_friends.data.len());
let zeros=Array::new(0);
printloop(zeros.data);

println!("//**************************************************************************************************");
println!("// F64 ARRAY");
println!("//**************************************************************************************************");

println!("//**************************************************************************************************");
println!("// STRUCT WITH MULTIPLE GENERIC TYPES (T,U)");
println!("//**************************************************************************************************");

println!("//**************************************************************************************************");
println!("// ENUM WITH GENERIC 2 TYPES");
println!("//**************************************************************************************************");

println!("//**************************************************************************************************");
println!("// TRAIT");
println!("//**************************************************************************************************");
let cheche:Person=Person{
    name:"CheChe".to_string(),
    age:23 as u32,
    occupation:Occupation::Programmer("No Job".to_string()),
    tierlist:TierList::F
};
let elon_musk:CEO<4>=CEO{
    name:"Elon Musk".to_string(),
    age:52 as u32,
    field:"Technology".to_string(),
    workers:[
Person{
    name:"Mumu".to_string(),
    age:26 as u32,
    occupation:Occupation::Scientist("Biologist".to_string()),
    tierlist:TierList::A
},
Person{
    name:"Tata".to_string(),
    age:20 as u32,
    occupation:Occupation::Engineer("Civil Engineer".to_string()),
    tierlist:TierList::A
},
Person{
    name:"Khem".to_string(),
    age:21 as u32,
    occupation:Occupation::Programmer("ML Engineer".to_string()),
    tierlist:TierList::A
},
Person{
    name:"Phoom".to_string(),
    age:22 as u32,
    occupation:Occupation::Programmer("Rust Big O Optimizer".to_string()),
    tierlist:TierList::A
},
]
};
println!("cheche.say_my_name() = {}",cheche.say_my_name());
println!("elon_musk.say_my_name() = {}",elon_musk.say_my_name());
println!("avatar_friends.say_my_name() = {}",avatar_friends.say_my_name());
println!("{}",zeros.say_my_name());
println!("//**************************************************************************************************");
println!("// ???");
println!("//**************************************************************************************************");

}

/*
cargo run
*/
