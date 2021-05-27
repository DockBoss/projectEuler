fn main() {
    let mut sum: i64 = 0;
    for n in 0..1000 {
            if n %3 == 0 || n % 5 == 0 || (n % 3 == 0 && n % 5 == 0) {
                sum = sum + n;
            }
        }
        println!("{}", sum);
    }