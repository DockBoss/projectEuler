fn main() {
    let mut sum: i64 = 0;
    let mut n1: i64 = 1;
    let mut n2: i64 = 1;
    let mut n3: i64 = 0;
    while n1 < 4_000_000 {
        n3 = n1;
        n1 += n2;
        n2 = n3;
        println!("N1 {}", n1);
        if n1 % 2 == 0 {
            sum += n1;
            println!("even {}", n1);
        }

    }
    println!("{}", sum)
}
