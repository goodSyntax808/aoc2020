use crate::read_nums;
use std::collections::HashSet;

fn find_product() -> Option<isize> {
    let mut prev_nums: HashSet<isize> = HashSet::new();
    let nums = read_nums("resources/day1.txt").unwrap();
    for num in nums {
        let complement = 2020 - num;
        if prev_nums.contains(&complement) {
            return Some(num * complement);
        }
        prev_nums.insert(num);
    }
    None
}

fn find_product2() -> Option<isize> {
    let mut nums = read_nums("resources/day1.txt").unwrap();

    for num in nums.iter() {
        for num_inner in nums.iter() {
            for num_inner_inner in nums.iter() {
                if num + num_inner + num_inner_inner == 2020 {
                    return Some(num * num_inner * num_inner_inner);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_task1() {
        let x = find_product();
        dbg!(x);
        assert_eq!(true, true);
    }

    #[test]
    fn run_task2() {
        let y = find_product2();
        dbg!(y);
    }
}
