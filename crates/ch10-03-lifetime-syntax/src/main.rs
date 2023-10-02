// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let str1 = String::from("adnd");
//     let str2 = "xyz";

//     let res = longest(str1.as_str(), str2);

//     println!("{}", res);
// }

// struct ImportantExcerpt<'b> {
//     part: &'b str,
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }
// }

// fn main() {
//     let novel = String::from("call me ishmael. some years ago ...");

//     let first_sentence = novel.split('.').next().expect("could not found a '.'");

//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("announcement {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {}
