use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("should have been a file");
    let highest_red_cubes = 12;
    let highest_green_cubes = 13;
    let highest_blue_cubes = 14;
    let mut sum: i32 = 0;
    let mut idx = 1;
    let mut flag = true;
    for line in input.lines() {
        let parts = line.split(":");
        for (i, part) in parts.enumerate() {
            if i == 0 {
                continue;
            } else {
                let sets = part.split(";").collect::<Vec<&str>>();
                // println!("{}: {:?}", idx, sets);
                for set in sets {
                    let trimmed_set = set.trim();
                    let cubes = trimmed_set.split(",").collect::<Vec<&str>>();
                    let trimmed_cubes = cubes.iter().map(|c| c.trim()).collect::<Vec<&str>>();
                    for cube in trimmed_cubes {
                        let cube_parts = cube.split(" ").collect::<Vec<&str>>();
                        // println!("{:?}", cube_parts);
                        let count = cube_parts[0].parse::<i32>().unwrap();
                        let color = cube_parts[1];
                        if color == "red" {
                            if count > highest_red_cubes {
                                flag = false;
                                break;
                            }
                        } else if color == "green" {
                            if count > highest_green_cubes {
                                flag = false;
                                break;
                            }
                        } else if color == "blue" {
                            if count > highest_blue_cubes {
                                flag = false;
                                break;
                            }
                        }
                    }
                }
                if flag == true {
                    print!("{} ", idx);
                    sum += idx;
                }
                flag = true;

            }
        }
        println!("");
        idx += 1;
    }
    println!("sum: {}", sum);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("should have been a file");
    let highest_red_cubes = 12;
    let highest_green_cubes = 13;
    let highest_blue_cubes = 14;
    let mut sum: i32 = 0;
    let mut idx = 1;
    let mut flag = true;
    for line in input.lines() {
        let parts = line.split(":");
        for (i, part) in parts.enumerate() {
            if i == 0 {
                continue;
            } else {
                let sets = part.split(";").collect::<Vec<&str>>();
                // println!("{}: {:?}", idx, sets);
                for set in sets {
                    let trimmed_set = set.trim();
                    let cubes = trimmed_set.split(",").collect::<Vec<&str>>();
                    let trimmed_cubes = cubes.iter().map(|c| c.trim()).collect::<Vec<&str>>();
                    for cube in trimmed_cubes {
                        let cube_parts = cube.split(" ").collect::<Vec<&str>>();
                        // println!("{:?}", cube_parts);
                        let count = cube_parts[0].parse::<i32>().unwrap();
                        let color = cube_parts[1];
                        if color == "red" {
                            if count > highest_red_cubes {
                                flag = false;
                                break;
                            }
                        } else if color == "green" {
                            if count > highest_green_cubes {
                                flag = false;
                                break;
                            }
                        } else if color == "blue" {
                            if count > highest_blue_cubes {
                                flag = false;
                                break;
                            }
                        }
                    }
                }
                if flag == true {
                    print!("{} ", idx);
                    sum += idx;
                }
                flag = true;

            }
        }
        println!("");
        idx += 1;
    }
    println!("sum: {}", sum);
}
