use std::fs;

fn main() {
    let input = fs::read_to_string("input2.txt").expect("should have been a file");
    let mut sum: i64 = 0;
    let mut idx = 1;
    for line in input.lines() {
        let parts = line.split(":");
        for (i, part) in parts.enumerate() {
            if i == 0 {
                continue;
            } else {
                let sets = part.split(";").collect::<Vec<&str>>();
                for set in sets {
                    let mut min_green_cubes = i32::MAX;
                    let mut min_red_cubes = i32::MAX;
                    let mut min_blue_cubes = i32::MAX;
                    let trimmed_set = set.trim();
                    let cubes = trimmed_set.split(",").collect::<Vec<&str>>();
                    let trimmed_cubes = cubes.iter().map(|c| c.trim()).collect::<Vec<&str>>();
                    for cube in trimmed_cubes {
                        let cube_parts = cube.split(" ").collect::<Vec<&str>>();
                        let count = cube_parts[0].parse::<i32>().unwrap();
                        let color = cube_parts[1];
                        if color == "green" {
                            if count < min_green_cubes {
                                min_green_cubes = count;
                            }
                        } else if color == "red" {
                            if count < min_red_cubes {
                                min_red_cubes = count;
                            }
                        } else if color == "blue" {
                            if count < min_blue_cubes {
                                min_blue_cubes = count;
                            }
                        }
                    }

                    let multi: i64 = min_green_cubes as i64 * min_red_cubes as i64 * min_blue_cubes as i64;
                    sum += multi;
                }
            }
        }
        println!("");
        idx += 1;
    }
    println!("sum: {}", sum);
}
