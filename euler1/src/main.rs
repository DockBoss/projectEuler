//Project Euler problem1
// find the sum of all numbers divisable by 3 and 5 from a set of size n


use std::time::SystemTime;

fn main() {
let mut t1 = SystemTime::now();
   // First Attempt
    let n: u64 = 999;
    let mut total: u64 = 0;
    for num in 0..(n + 1) {
            if num %3 == 0 || num % 5 == 0 || (num % 3 == 0 && num % 5 == 0) {
                total += num;
            }
        }
        println!("First {}", total);
        let mut t2 = SystemTime::now();
        let difference = t2.duration_since(t1);
        println!("{:?}", difference);

//second Attempt 
            //after reading problem1 pdf on project euler
            //https://projecteuler.net/overview=001
        t1 = SystemTime::now();
       

       let total2 = sum_divisable_by(3, n) + sum_divisable_by(5, n) - sum_divisable_by(15, n);
      
        println!("Second {}", total2);
        t2 = SystemTime::now();
        let difference = t2.duration_since(t1);
        println!("{:?}", difference);
    }
    //used in second attempt
    fn sum_divisable_by(num: u64, n: u64) -> u64 { 
        let p  = n / num;
        return num * (p *(p + 1)) / 2;
    }