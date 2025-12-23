

struct Point{
    x : isize,
    y : isize,
    z : isize

}

impl Point {
    fn distance(&self, other_point : &Point)-> isize{
        return self.x * other_point.x + self.y * other_point.y + self.z * other_point.z;
    }   
}


struct PointPair{
    first : Point,
    second : Point, 
}

pub fn part2(){
    let input = std::fs::read_to_string("input/day8.txt").expect("");
    let lines = input.lines();
    let mut points = Vec::<Point>::new();
    for line in lines{
        println!("{}", line);
        let cell : Vec<&str> = line.split(",").collect();
        let point = Point{
            x : cell[0].parse::<isize>().unwrap(),
            y: cell[1].parse::<isize>().unwrap(),
            z: cell[2].parse::<isize>().unwrap(),
        };
        points.push(point);
    }
    let all_point_pairs = points.iter().flat_map(|p1| {
        points.iter().map(|p2| PointPair{
            first: Point{
                x: p1.x,
                y: p1.y,
                z: p1.z,
            },
            second: Point{
                x: p2.x,
                y: p2.y,
                z: p2.z,
            },
        })
    });
}