fn main() {
    let value:u64 = 600851475143;
    let mut flag = true;
    let mut i = 2;
    let mut largest_prime = 1;
    
    while flag {
        if i * i <= value {
            if value % i == 0 && prime_checker(i) {
                largest_prime = i;
            }
        }
        else{
            flag = false;
        }
        i += 1;
    }
    println!("{}", largest_prime);
}

fn prime_checker(value: u64) -> bool {
    let mut i = 2;

    loop {
        if i * i > value {
            return true;
        }
        if value % i == 0 {
            return false;
        }
        i += 1;
    }
}