fn main() {
    println!("Hello, world!!!");

    let mut slice = [1, 2, 3, 4, 5];

    {
        let (left, right) = slice.split_at_mut(3);
        left[0..2].copy_from_slice(&right[0..2]);
    }

    println!("slice {:?}", &slice);

    // assert_eq!(slice, [4, 5, 3, 4, 5]);
}
