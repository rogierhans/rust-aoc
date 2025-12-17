


pub fn part2(){
    let filename = "input/day3.txt";
    let input_lines = std::fs::read_to_string(filename).expect("Lmao");
    let lines = input_lines.split("\n").filter(|line: &&str| (*line) != "");

    let mut total :i64= 0;
    for line in lines{
        let numbers: Vec<i8> = line.split("").filter_map(|part: &str| part.parse::<i8>().ok()).collect();
        let length = numbers.len();
        let mut highest_index = 0;
        let mut result_vec = Vec::<i8>::new();
        for keep_number in [11,10,9,8,7,6,5,4,3,2,1,0]{
            let mut best_index =-1;
            let mut best_value = -1;
            for index_number in highest_index..(length-keep_number){
                if numbers[index_number as usize] > best_value{
                    best_value = numbers[index_number as usize];
                    best_index = index_number as i8;
                }
            }
            highest_index = (best_index +1) as usize;
            result_vec.push(best_value);   

        }
        let mut result_string = String::new();
        for number in result_vec{
            result_string.push_str(&number.to_string());
        }
        total += result_string.parse::<i64>().unwrap();

    }
    println!("Total: {}", total);
} 