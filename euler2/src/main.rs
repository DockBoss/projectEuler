//Project Euler problem2
//find the sum of the even numbers of the Fibonacci sequence less than 4_000_000

use std::time::SystemTime;

fn main() {
    //second Attempt
        //after reading https://projecteuler.net/overview=002
        //runs between   microseconds
}

//first Attempt
// it works, probably slow 
// runs between 150-300 microseconds 

//let t1 = SystemTime::now();
//     let mut sum: i64 = 0;
//     let mut n1: i64 = 1;
//     let mut n2: i64 = 1;
//     let mut n3: i64 = 0;
//     while n1 < 4_000_000 {
//         n3 = n1;
//         n1 += n2;
//         n2 = n3;
//         //println!("N1 {}", n1);
//         if n1 % 2 == 0 {
//             sum += n1;
//             //println!("even {}", n1);
//         }

//     }
//     println!("{}", sum);
//     let t2 = SystemTime::now();
//     let dif = t2.duration_since(t1);
//     println!("{:?}", dif);
// }
