fn main() {
    let mut sum = 2;
    let mut counter = 0;
    let mut curr = 2;
    let mut prev = 1;
    let ceiling = 4000000;

    while curr < ceiling {
        if counter == 3 {
            counter = 1;
            sum += curr;
        }
        else{
            counter += 1;
        }
        let tmp = curr;
        curr = prev + curr;
        prev = tmp;
    }

    proof(sum, ceiling);

    println!("{}", sum.to_string());
}

fn proof(sum_to_be_proven : i32, ceiling : i32) {
    let mut prev = 0;
    let mut curr = 1;
    let mut sum = 0;

    while curr < ceiling {
        if curr % 2 == 0 {
            sum += curr;
        }
        let tmp = curr;
        curr = prev + curr;
        prev = tmp;
    }

    assert_eq!(sum, sum_to_be_proven);
}
