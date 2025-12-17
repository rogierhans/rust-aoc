

pub fn part2() {
    //read input file
    let input = std::fs::read_to_string("input/day1.txt").expect("Failed to read input file");
    // get the first char of every line
    let chars= input.lines().map(|line| line.chars().next());
    // the numbers are in the line after the char
    let numbers  =input.lines().map(|line| line[1..].parse::<i32>().ok());
    // zip the two vectors together
    let mut start: i32 = 50;
    let mut at_zero: i32 = 0;
    for (c, n) in chars.zip(numbers) {
        let n = n.unwrap_or(0);
        if c == Some('L') {
            for _ in 0..n {
                start -= 1;
                start = (start + 100) % 100; // wrap around
                if start == 0 {
                    at_zero += 1;
                }
            }
        } else if c == Some('R') {
            for _ in 0..n {
                start += 1;
                start = (start + 100) % 100; // wrap around
                if start == 0 {
                    at_zero += 1;
                }
            }
        }
    }
    print!("At zero: {}\n", at_zero);

}
