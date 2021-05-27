//Project Euler problem1
// find the sum of all numbers divisible by 3 and 5 from a set of size 999

fn main() {
    
//second Attempt 
            //after reading problem1 pdf on project euler
            //https://projecteuler.net/overview=001
            //almost twice as fast, because it does like 6 math operations instead comparing every number from 0 to 999
        let n: u64 = 999;

        let total = sum_divisible_by(3, n) + sum_divisible_by(5, n) - sum_divisible_by(15, n);
        println!("Second {}", total);
    }
  
    //Returns the sum of all numbers from 0 to n that are divisible by num
    fn sum_divisible_by(num: u64, n: u64) -> u64 { 
        let p  = n / num;
        return num * (p *(p + 1)) / 2;
    }


     // First Attempt
            //rather slow
            //probably because of the long conditional statement that needs to be checked each iteration
            //where second time just does 3 separate calculations    

    //  for num in 0..(n + 1) {
    //     if num %3 == 0 || num % 5 == 0 || (num % 3 == 0 && num % 5 == 0) {
    //         total += num;
    //     }
    // }
    // println!("First {}", total);