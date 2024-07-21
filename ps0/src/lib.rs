pub fn solve_01a(a: Vec<u32>, b: Vec<u32>) -> Vec<u32> {
    let mut result = vec![];
    for element in a {
        if b.contains(&element) {
            result.push(element);
        }
    }
    result
}

pub fn solve_01b(a: Vec<u32>, b: Vec<u32>) -> u32 {
    let mut result = a;
    for element in b {
        if !result.contains(&element) {
            result.push(element);
        }
    }
    result.len().try_into().unwrap_or(0)
}

pub fn solve_01c(a: Vec<u32>, b: Vec<u32>) -> u32 {
    let mut result = a;
    for element in b {
        if result.contains(&element) {
            result.retain(|&x| x != element);
        }
    }
    result.len().try_into().unwrap_or(0)
}

pub fn solve_02a() -> f32 {
    // X = Number of heads after flipping a coin 3 times
    // E[X]
    let prob_heads = 0.5;
    let num_flips = 3.;
    prob_heads * num_flips
}

pub fn solve_02b() -> f32 {
    // Y = Rolling a fair 6-sided dice two times and multiplying the values
    // E[Y]
    let prob: f32 = (1. / 6.) * (1. / 6.);
    let mut result = 0.0;
    for i in 1..=6 {
        for j in 1..=6 {
            result += prob * i as f32 * j as f32;
        }
    }
    result
}

pub fn solve_02c() -> f32 {
    // E[X + Y] = E[X] + E[Y] mutually exclusive
    solve_02a() + solve_02b()
}

pub fn solve_03a(a: i32, b: i32) -> bool {
    let modulu = 2;
    a % modulu == b % modulu
}
pub fn solve_03b(a: i32, b: i32) -> bool {
    let modulu = 3;
    a % modulu == b % modulu
}
pub fn solve_03c(a: i32, b: i32) -> bool {
    let modulu = 4;
    a % modulu == b % modulu
}

pub fn solve_06(a: Vec<i32>) -> i32 {
    let mut largest_length = 1;
    let mut largest_length_count = 0;
    let mut current_length = 1;
    let mut last_value = None;
    for i in a {
        if last_value.is_some() {
            if i > last_value.unwrap() {
                current_length += 1;
                if current_length > largest_length {
                    largest_length = current_length;
                    largest_length_count = 1;
                } else if current_length == largest_length {
                    largest_length_count += 1;
                }
            } else {
                current_length = 1;
            }
            last_value = Some(i);
        } else { last_value = Some(i) }
    }
    largest_length_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_01a() {
        let a = vec![1, 6, 12, 13, 9];
        let b = vec![3, 6, 12, 15];
        let result = solve_01a(a, b);
        assert_eq!(result, vec![6, 12]);
    }

    #[test]
    fn problem_01b() {
        let a = vec![1, 6, 12, 13, 9];
        let b = vec![3, 6, 12, 15];
        let result = solve_01b(a, b);
        assert_eq!(result, 7);
    }

    #[test]
    fn problem_01c() {
        let a = vec![1, 6, 12, 13, 9];
        let b = vec![3, 6, 12, 15];
        let result = solve_01c(a, b);
        assert_eq!(result, 3);
    }

    #[test]
    fn problem_02a() {
        let result = solve_02a();
        assert_eq!(result, 1.5);
    }

    #[test]
    fn problem_02b() {
        let result = solve_02b();
        assert_eq!(result, 12.25);
    }

    #[test]
    fn problem_02c() {
        let result = solve_02c();
        assert_eq!(result, 13.75);
    }

    #[test]
    fn problem_03a() {
        let a = 600 / 6;
        let b = 60 % 42;
        let result = solve_03a(a, b);
        assert_eq!(result, true);
    }

    #[test]
    fn problem_03b() {
        let a = 600 / 6;
        let b = 60 % 42;
        let result = solve_03b(a, b);
        assert_eq!(result, false);
    }

    #[test]
    fn problem_03c() {
        let a = 600 / 6;
        let b = 60 % 42;
        let result = solve_03c(a, b);
        assert_eq!(result, false);
    }

    #[test]
    fn problem_06() {
        let tests = [
            (
                vec![2, 2, 4, 1, 4],
                2,
            ),
            (
                vec![7, 8, 5, 7, 7, 3, 2, 8],
                3,
            ),
            (
                vec![7, 7, 9, 1, 2, 11, 9, 6, 2, 8, 9],
                2,
            ),
            (
                vec![4, 18, 10, 8, 13, 16, 18, 1, 9, 6, 11, 13, 12, 5, 7, 17, 13, 3],
                1,
            ),
            (
                vec![11, 16, 10, 19, 20, 18, 3, 19, 2, 1, 8, 17, 7, 13, 1, 11, 1, 18, 19, 9, 7, 19, 24, 2, 12],
                4,
            ),
        ];

        for test in tests {
            println!("{:?}", test);
            assert_eq!(solve_06(test.0), test.1);
        }
    }
}