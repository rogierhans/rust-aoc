

fn transpose(lines : &Vec<&str>) -> Vec<String> {
    let mut transposed: Vec<String> = Vec::new();
    if lines.is_empty() {
        return transposed;
    }
    let line_length = lines[0].len();
    for _ in 0..line_length {
        transposed.push(String::new());
    }
    for line in lines {
        for (i, ch) in line.chars().enumerate() {
            transposed[i].push(ch);
        }
    }       
    transposed
}

fn split_vector(input_lines :Vec<String>)-> Vec<Vec<String>> {
    let mut groups : Vec<Vec<String>> = Vec::<Vec<String>>::new();
    let mut current_group :Vec<String> = Vec::new();
    for line in input_lines {
        if line.trim().is_empty() {
            if !current_group.is_empty() {
                groups.push(current_group);
                current_group = Vec::new();
            }
        } else {
            current_group.push(line);
        }
    }
    if !current_group.is_empty() {
        groups.push(current_group);
    }
    groups
}


pub fn part2() {
    //read input file
    let input = std::fs::read_to_string("input/day6.txt").expect("Failed to read input file");
    let lines = input.lines().collect::<Vec<&str>>();
    let transposed = transpose(&lines);
    let groups = split_vector(transposed);
    let mut total:i128 = 0;
    for (i, group) in groups.iter().enumerate() {
        let last_char_of_first = group[0].chars().last().unwrap();
        let numbers: Vec<i128> = group.iter().map(|line| line.replace("*", "").replace("+", "").trim().parse::<i128>().unwrap_or(0)).collect();
        if last_char_of_first == '+' {
            total += numbers.iter().sum::<i128>();
        }
        else {
            total +=  numbers.iter().product::<i128>();

        }

    }
    println!("Total: {}", total);

}
