use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("should have been a file");

    // here we will replace the words with the number like one to o1e two to t2o and so oe
    let input = input.replace("one", "o1e");
    let input = input.replace("two", "t2o");
    let input = input.replace("three", "t3e");
    let input = input.replace("four", "f4r");
    let input = input.replace("five", "f5e");
    let input = input.replace("six", "s6x");
    let input = input.replace("seven", "s7n");
    let input = input.replace("eight", "e8t");
    let input = input.replace("nine", "n9e");

    let mut sum: i32 = 0;
    for line in input.lines() {
        println!("The line is: {}", line);
        let mut first: char = ' ';
        let mut last: char = ' ';
        for chrs in line.chars() {
            if chrs.is_digit(10) {
                if first == ' ' {
                    first = chrs;
                    last = chrs;
                } else {
                    last = chrs;
                }
            }
        }
        let mut string_ans: String = String::from(first)  ;
        string_ans += &String::from(last);
        println!("The string is: {}", string_ans);
        let string_ans_final = string_ans.parse::<i32>().unwrap();
        sum += string_ans_final;
    }
    println!("The sum is: {}", sum)
}
