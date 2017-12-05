fn main() {
    for a in 1..1000 {
        let a = a as usize;
        for b in 2..1001 {
            let b = b as usize;
            if a >= b {
                continue;
            }
            let c = ((a.pow(2) + b.pow(2)) as f64).sqrt() as usize;
            if a + b + c == 1000 && a.pow(2) + b.pow(2) == c.pow(2) {
                println!("{} * {} * {} =", a, b, c);
                println!("{}", a * b * c);
                return;
            }
        }
    }
    println!("not found");
}
