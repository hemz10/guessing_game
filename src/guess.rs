// use core::num;
// use rand::Rng;
// use std::cmp;
use std::{io};

//Guessing Game
// pub fn guess(){
//     let mut take:i32 = 0;
//     let secret_number: u32 = rand::thread_rng().gen_range(0..101);
//     // println!("Secret Number: {}", secret_number);
//     loop {
//         take += 1;
//         let mut guess: String = String::new();
//         println!("Guess a Number");
//         io::stdin().read_line(&mut guess).expect("Enter Number: ");
//         println!("Guessed Number: {}", guess);
//         let guess: u32 = guess.trim().parse().expect("Enter Number only");
//         match guess.cmp(&secret_number) {
//             cmp::Ordering::Less => println!("Entered Number is lower than secret_number"),
//             cmp::Ordering::Equal => {
//                 println!("You won");
//                 break;
//             }
//             cmp::Ordering::Greater => println!("Entered Number is greater than secret_number"),
//         }
//     }
//     println!("Number of takes:{}", take)
// }

// // 100 Years
// pub fn guess(){
//     let mut name:String = String::new();
//     println!("Enter your Name: ");
//     io::stdin().read_line(&mut name).unwrap();
//     let mut age:String = String::new();
//     println!("Enter your age: ");
//     io::stdin().read_line(&mut age).unwrap();
//     let age:u32 = age.trim().parse().unwrap();
//     let current_year:u32 = 2022;
//     let hundred_years:u32 = current_year + (100 - age);
//     print!("{} you will be hundred in the year {}", name, hundred_years);
// }

// Odd or Even number
// pub fn guess() {
//     let mut number = String::new();
//     println!("Enter Number: ");
//     io::stdin().read_line(&mut number).unwrap();
//     let number: i32 = number.trim().parse().unwrap();
//     if number % 2 == 0 {
//         println!("Its a Even number");
//     } else {
//         println!("Its a Odd number");
//     }
// }

// Print number less than 5
// pub fn guess(){
//     let a:[i32; 11] = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
//     for number in a{
//         if number < 5 {
//             println!("{}",number);
//     }
//   }
// }

// // To find all Divisors
// pub fn guess(){
//     let mut dividend = String::new();
//     println!("Enter a number to find all divisors");
//     io::stdin().read_line(&mut dividend).unwrap();  
//     let dividend:u32 = dividend.trim().parse().unwrap();
//     let mut all_divisors = Vec::new();
//     for number in 1..dividend+1{
//         if dividend % number == 0 {
//             all_divisors.push(number);
//         }
//     }
//     println!("{:?}", all_divisors);
// }


// To merge and print without duplicates  
// pub fn guess(){

//     let mut a = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
//     let mut b = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
//     a.append(&mut b);
//     a.sort();
//     a.dedup();
//     println!("{:?}",a);
// }

pub fn guess(){
    let mut text = String::new();
    println!("Enter a text to find if its palindrome: ");
    io::stdin().read_line(&mut text).unwrap();
    let text = text.to_lowercase();
    let a = text.trim();
    let array = a.chars().collect::<Vec<char>>();
    let mut rev = array.clone();
    rev.reverse();
    let a :String= array.iter().collect();
    let b : String = rev.iter().collect();
    if a==b{
        println!("palindrome");
    }
    else{
        println!("not palindrome")
    }

}