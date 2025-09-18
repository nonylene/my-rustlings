// TODO: Fix the compiler error in the function without adding any new line.
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec.to_vec()
}

fn test<'a>(mut vec: &'a mut Vec<i32>, _vec2: &'a mut Vec<i32>) {
    // original vec in main() should not change because this is just a reference only in this scope (vec is mutable variable whose value (aka pointer) is copied from arg)
    vec = _vec2;
}

fn main() {
    let mut x = vec![1, 2, 3];
    let mut y = vec![1, 2, 4];
    test(&mut x, &mut y);
    println!("{}", x[2]);
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
