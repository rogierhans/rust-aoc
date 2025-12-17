

struct Range {
    start: i64,
    end: i64,
}

pub fn part2() {
    //read input file
    let input = std::fs::read_to_string("input/day5.txt").expect("Failed to read input file");
    let sections = input.lines();
    //print sections
    let mut ranges : Vec<Range> = Vec::new();
    for section in sections {
        let numbers  : Vec<&str> = section.trim().split("-").collect();
        let first = numbers.get(0).unwrap_or(&"0").parse::<i64>().unwrap_or(0);
        let second = numbers.get(1).unwrap_or(&"0").parse::<i64>().unwrap_or(0);
        ranges.push(Range { start: first, end: second });
    }
    //sort ranges by first element
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let smallest_start  = ranges.first().unwrap().start;

    let largest_end = ranges.iter().map(|r| r.end).max().unwrap_or(0);
    let mut difference = largest_end - smallest_start +1;

    let mut current_end = ranges.first().unwrap().end;
    for range in ranges.iter().skip(1) {
        if range.start > current_end + 1 {
            // there is a gap
            let gap_size = range.start - current_end - 1;
            difference -= gap_size;
        }
        if range.end > current_end {
            current_end = range.end;
        }
    }
    println!("Total covered range size: {}", difference);



}
