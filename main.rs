fn main() {
    println!("2^3 = {}", pow(2, 3));
    println!("2^8 = {}", pow(2, 8));
}

fn pow(value: i64, power: i64) -> i64 {
    if power == 0 {
        return 1;
    }
    if power == 1 {
        return value;
    }
    
    let mut res = value;
    for _ in 1..power {
        println!("res = {} * {}", res, value);
        res *= value;
    }
    
    res
}

