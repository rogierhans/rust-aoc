
pub fn parse_line(line:&str )-> Vec<bool>{
    return  line.chars().map(|c| c == 'c').collect();
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day7.txt").expect("");
    let lines = input.split("\n").collect::<Vec<&str>>();
    let parsed_lines:Vec<Vec<bool>> = lines.iter().map(|line| parse_line(line)).collect();
    for line in parsed_lines {
        println!("{}",line)

    }


}