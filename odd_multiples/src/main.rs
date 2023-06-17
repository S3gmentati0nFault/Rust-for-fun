fn main() {
    let mut sum = 0;
    let mut i = 3;
    let mut j = 0;
    let ceiling= 1000;


    while i < ceiling {
        if j == 4 {
            j = -1;
        }
        else{
            sum += i;
        }
        i += 3;
        j += 1;
    }

    i = 5;

    while i < ceiling {
        sum += i;
        i += 5;
    }

    proof(sum, ceiling);

    println!("{}", sum.to_string());
}

fn proof(sum : i32, ceiling : i32) {
    let mut proof_sum = 0;

    for i in 1..ceiling {
        if i % 3 == 0 || i % 5 == 0 {
            proof_sum += i;
        }
    }

    assert_eq!(proof_sum, sum);
}