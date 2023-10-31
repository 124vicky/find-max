fn find_max<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None
    }

    let mut max = &list[0];
    for item in list.iter() {
        if item > max {
            max = item;
        }
    }

    Some(max)
}

fn main() {
    let l1 = [1,6,10];
    let max = find_max(&l1).unwrap().to_owned();
    println!("max in l1:{}", max);

    let l2:[_; 0] = [];
    let max2 = find_max::<Option<&u32>>(&l2);
    println!("max in l2:{:?}", max2);

}