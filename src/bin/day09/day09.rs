use core::panic;

fn main() {
    let input = advent2020::read_input("input09.txt");

    let mut nums: Vec<i64> = Vec::new();
    for line in input.split("\n") {
        nums.push(line.parse().unwrap())
    }

    let p1 = part1(&nums);
    println!("Part 1 - {}", nums[p1]);
    println!("Part 2 - {}", part2(&nums, p1));
}

fn part1(nums: &Vec<i64>) -> usize {
    let mut previous = &nums[0..25];

    for i in 25..nums.len() {
        if !can_sum_to(previous, nums[i]) {
            return i;
        }

        previous = &nums[i - 24..i + 1]
    }

    panic!("reached end of numbers!")
}

fn can_sum_to(from: &[i64], sum: i64) -> bool {
    // println!("\t{:?}, {}", from, sum);
    let l = from.len();
    for i in 0..l - 1 {
        for j in i + 1..l {
            if from[i] + from[j] == sum {
                // println!("{} + {} = {}", from[i], from[j], sum);
                return true;
            }
        }
    }

    false
}

fn part2(nums: &Vec<i64>, goal: usize) -> i64 {
    for i in 0..goal {
        for j in i..goal {
            let mut sum = 0;
            let mut smallest:i64 = 99999999999999999;
            let mut largest:i64 = 0;

            for k in i..j {
                if smallest > nums[k] {
                    smallest = nums[k];
                }

                if largest < nums[k] {
                    largest = nums[k];
                }

                sum += nums[k]
            }
            if sum == nums[goal] {
                return largest + smallest;
            }
        }
    }
    panic!("reached end of numbers!")
}
