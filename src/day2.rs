


fn check_number_k(str_number:&str, k: usize) -> bool{

    let length_str = str_number.len();
    if length_str % k == 0 {
        let section_length = length_str / k;
        let first_part = &str_number[0..section_length];
        for i in 1..k{
            let part = &str_number[i*section_length..(i+1)*section_length];
            if part != first_part{
                return false;
            }
        }
        return true;
    }
    return false;
}

fn check_range(start : i64, end: i64)-> i64{
    let mut count = 0;
    for i in start..end{
        let str_number= i.to_string();
        let str_reference = str_number.as_str();
        for k in 2..8{
            if check_number_k(str_reference, k){
                count += i;
                break;
            }
        }
    }
    return count;
}


pub fn part2 (){
    let filename =  "/home/tony/Rust/aoc/input/day2.txt";
    let line = std::fs::read_to_string(filename).expect("lmao");
    let sections : Vec<&str>= line.split(",").collect();
    let mut total = 0;
    for section in sections{
        let parts : Vec<&str> = section.trim().split("-").collect();
        let start : i64 = parts[0].parse().unwrap();
        let end : i64 = parts[1].parse().unwrap();
        total += check_range(start, end);

    }
    println!("{}",total   );

}