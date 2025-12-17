

pub fn count_true(array: &Vec<bool>) -> usize {
    let mut count = 0;
    for &value in array.iter() {
        if value {
            count += 1;
        }
    }
    count
}

pub fn part2(){
    let filename = "input/day4.txt";
    let input_line = std::fs::read_to_string(filename).expect("Lmao");
    let lines= input_line.split("\n").collect::<Vec<&str>>();
    let length = lines.len();
    let length_first_line = lines[0].len();
    let mut array: Vec<bool> = vec![false; length*length_first_line];
    for i in 0..length{
        let line = lines[i];
        for (j, ch) in line.chars().enumerate(){
            let index = i*length_first_line + j;
            array[index] = ch == '@';
        }
    }

    let inital_true_count = count_true(&array);
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..length{
            for j in 0..length_first_line{
                let index = i*length_first_line + j;
                if array[index] {
                    //check neighbors
                    let mut count_neighbors = 0;
                    for di in -1..=1{
                        for dj in -1..=1{
                            if di == 0 && dj == 0 {
                                continue;
                            }
                            let ni = i as isize + di;
                            let nj = j as isize + dj;
                            if ni >= 0 && ni < length as isize && nj >= 0 && nj < length_first_line as isize {
                                let nindex = (ni as usize)*length_first_line + (nj as usize);
                                if array[nindex] {
                                    count_neighbors += 1;
                                }
                            }
                        }
                    }
                    if count_neighbors < 4 {
                        array[index] = false;
                        changed = true;
                    }
                }
            }
        }
    }


    let final_true_count = count_true(&array);
    println!("Initial true count: {}", inital_true_count);
    println!("Final true count: {}", final_true_count);
    let result = inital_true_count - final_true_count;
    println!("Result: {}", result);
} 