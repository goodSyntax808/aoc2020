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
    let mut prev_nums: HashSet<isize> = HashSet::new();
    let nums = read_nums("resources/day1.txt").unwrap();
    nums.retain(|&x| x
    nums.iter()
        
    for num in nums {
        let complement = 2020 - num;
        if prev_nums.contains(&complement) {
            return Some(num * complement);
        }
        prev_nums.insert(num);
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
    fn run_task2() {}
}
