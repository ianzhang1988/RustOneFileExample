
fn sum_to(n: u32) -> u32{
    // 1..n Range<u32> give a iterator
    (1..n).fold(0, |sum, item| {sum + item})
}

fn test_into_iter() {
    let v1 = vec![1,2,3,4];
    let v2 = vec![1,2,3,4];

    println!("v1 len: {}", v1.len());

    for i in &v1 { // (&v1).into_iter()
        let _dummy = i;
    }

    println!("v1 len: {}", v1.len());


    println!("v2 len: {}", v2.len());
    for i in v2.iter() {
        let _dummy = i;
    }
    println!("v2 len: {}", v2.len());

    // looks like these two (&v1).into_iter() v2.iter() are the same
    // in programing rust page 327, .iter() clearer for non loop case, IntoIterator Trait
    // can be used in where clause in generic code
}

fn find_prime(n:u32) {
    let prime_nums: Vec<u32> = (1..n).filter( | i | {
        let i_tmp = *i as f32;
        !(2..i_tmp.sqrt().ceil() as u32 + 1).any(| j | { i % j == 0 })
    } ).collect();
    println!("prime_nums {:?}", prime_nums);
}


fn main() {
    println!("sum {}", sum_to(10));
    test_into_iter();
    find_prime(100);
}