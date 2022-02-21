// fn main() {
//     let x = 5;
//     let y = &x;

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// //  assert_eq!(5, y);
// }

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);

//     print!("s is {}",s)
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// #![allow(unused)]
// fn main() {
// let mut s = String::from("hello");

// let r1 = &mut s;
// let r2 = &mut s;

// println!("{}, {}", r1, r2);
// }

// #![allow(unused)]
// fn main() {
//     let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//     } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

//     let r2 = &mut s;
// }


// #![allow(unused)]
// fn main() {
// let mut s = String::from("hello");

// let r1 = &s; // 没问题
// let r2 = &s; // 没问题
// let r3 = &mut s; // 大问题

// println!("{}, {}, and {}", r1, r2, r3);
// }

fn main() {
    let mut s = String::from("hello");
 
     let r1 = &s; 
     let r2 = &s; 
     println!("{} and {}", r1, r2);
     // 新编译器中，r1,r2作用域在这里结束
 
     let r3 = &mut s; 
     println!("{}", r3);
 } // 老编译器中，r1、r2、r3作用域在这里结束
   // 新编译器中，r3作用域在这里结束
 

// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
