/**
 * Read the input file and return a vector of strings
 */
fn read_input_file() -> Vec<String> {
    std::fs::read_to_string("src/joltages.txt").unwrap().lines().map(|x| x.to_string()).collect()
}

/**
 * Take an input like 4735243252264454742487242145543733424454475231547442975364451544354542541564536434255546274455744268
 * and return a vector of i32 like [4, 7, 3, 5, 2, 4, 3, 2, 5, 2, 2, 6, 4, 4, 5, 4, 7, 4, 2, 4, 8, 7, 2, 4, 2, 1, 4, 5, 5, 4, 3, 7, 3, 3, 4, 2, 4, 4, 5, 4, 4, 7, 5, 2, 3, 1, 5, 4, 7, 4, 4, 2, 9, 7, 5, 3, 6, 4, 4, 5, 1, 5, 4, 4, 3, 5, 4, 5, 4, 2, 5, 4, 1, 5, 6, 4, 5, 3, 6, 4, 3, 4, 2, 5, 5, 5, 4, 6, 2, 7, 4, 4, 5, 5, 7, 4, 4, 2, 6, 8]
 */
fn parse_input(input: String) -> Vec<i32> {
    input.chars().map(|x| x.to_digit(10).unwrap() as i32).collect()
}

/**
* Finds the two biggest numbers in a vector from left to right and returns their concatenation, e.g. from a list like 811111111111119 the concatenation is 89.
*/
fn get_max_joltage(joltages: &Vec<i32>) -> i32 {
    let mut max_joltage = 0;
    let mut second_max_joltage = 0;
    // first find the max joltage by searching from the start to n - 1 for the biggest number
    for joltage in joltages.iter().take(joltages.len() - 1) {
        if *joltage > max_joltage {
            max_joltage = *joltage;
        }
    }

    let index = joltages.iter().position(|&x| x == max_joltage).unwrap();
    // then find the second max joltage by searching from the max joltage to the end of the vector for the next biggest number
    for joltage in joltages.iter().skip(index + 1) {
        if *joltage > second_max_joltage {
            second_max_joltage = *joltage;
        }
    }
    max_joltage * 10 + second_max_joltage
}

fn get_12_max_joltage(joltages: &Vec<i32>) -> i64 {
    let k = 12usize;
    let n = joltages.len();
    if k == 0 || n == 0 {
        return 0;
    }

    let mut stack: Vec<i32> = Vec::with_capacity(k);

    for i in 0..n {
        let v = joltages[i];

        // while we can pop and still fill k elements afterwards (including current),
        // and the last element is smaller than current, pop it to make a bigger number
        while !stack.is_empty()
            && *stack.last().unwrap() < v
            && stack.len() + (n - i) > k
        {
            stack.pop();
        }

        if stack.len() < k {
            stack.push(v);
        }
    }

    // concatenate digits into i64
    stack.into_iter().fold(0i64, |acc, d| acc * 10 + d as i64)
}

fn sum_max_joltages(joltages: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for joltage in joltages {
        sum += joltage;
    }
    sum
}

fn main() {
    let input = read_input_file();
    let mut joltages = Vec::new();
    for joltage in input {
        let parsed_joltages = parse_input(joltage);
        let max_joltage = get_12_max_joltage(&parsed_joltages);
        println!("Max joltage: {}", max_joltage);
        joltages.push(max_joltage);
    }
    let sum_max_joltages = sum_max_joltages(&joltages);
    println!("Sum of max joltages: {}", sum_max_joltages);
}