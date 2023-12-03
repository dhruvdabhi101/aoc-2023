use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("should have been a file");
    let mut sum: i64 = 0;
    let mut idx = 1;
    for line in input.lines() {
        let parts = line.split(":");
        for (i, part) in parts.enumerate() {
            if i == 0 {
                continue;
            } else {
                let sets = part.split(";").collect::<Vec<&str>>();
                let mut max_green_cubes = 0;
                let mut max_red_cubes = 0;
                let mut max_blue_cubes = 0;
                for set in sets {
                    let trimmed_set = set.trim();
                    let cubes = trimmed_set.split(",").collect::<Vec<&str>>();
                    let trimmed_cubes = cubes.iter().map(|c| c.trim()).collect::<Vec<&str>>();
                    // println!("{}: {:?}", idx, trimmed_cubes);
                    for cube in trimmed_cubes {
                        let cube_parts = cube.split(" ").collect::<Vec<&str>>();
                        let count = cube_parts[0].parse::<i32>().unwrap();
                        let color = cube_parts[1];
                        if color == "green" {
                            if count >= max_green_cubes {
                                max_green_cubes = count;
                            }
                        } else if color == "red" {
                            if count >= max_red_cubes {
                                max_red_cubes = count;
                            }
                        } else if color == "blue" {
                            if count >= max_blue_cubes {
                                max_blue_cubes = count;
                            }
                        }
                    }
                }
                // println!("{} {} {}", max_red_cubes, max_green_cubes, max_blue_cubes);
                let multi: i64 =
                    max_blue_cubes as i64 * max_red_cubes as i64 * max_green_cubes as i64;
                sum += multi;
            }
        }
        // println!("");
    }
    println!("sum: {}", sum);
}
