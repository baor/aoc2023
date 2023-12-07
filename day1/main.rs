use regex::Regex;
use std::fs::File;
use std::io::Read;

fn act1(input: &String) -> Vec<i32> {
    let values = input.split_whitespace().collect::<Vec<_>>();
    let mut output = Vec::new();
    let first_left = Regex::new(r"^\D*(\d)\w*$").unwrap();
    let first_right = Regex::new(r"\w*(\d)\D*$").unwrap();

    for v in values {
        //println!("value: {:?}", v);
        let Some(caps) = first_left.captures(v) else {
            println!("no match!");
            return output;
        };
        let mut number_str = String::new();
        number_str.push_str(&caps[1]);
        
        let Some(caps) = first_right.captures(v) else {
            println!("no match!");
            return output;
        };
        number_str.push_str(&caps[1]);
        output.push((number_str).parse::<i32>().unwrap());
    }
    output
}


fn act2(input: &String) -> Vec<i32> {
    let values = input.split_whitespace().collect::<Vec<&str>>();
    let mut output = Vec::new();
    let first_left = Regex::new(r"^\D*(\d)\w*$").unwrap();
    let first_right = Regex::new(r"\w*(\d)\D*$").unwrap();

    for v in values {
        println!("value: {:?}", v);
        let mut v_str = v.to_string();
        v_str = v_str.replace("one", "o1e");
        v_str = v_str.replace("two", "t2o");
        v_str = v_str.replace("three", "t3e");
        v_str = v_str.replace("four", "f4r");
        v_str = v_str.replace("five", "f5e");
        v_str = v_str.replace("six", "s6x");
        v_str = v_str.replace("seven", "s7n");
        v_str = v_str.replace("eight", "e8t");
        v_str = v_str.replace("nine", "n9e");
        println!("value after: {:?}", v_str);
        let Some(caps) = first_left.captures(&v_str) else {
            println!("no match!");
            return output;
        };
        let mut number_str = String::new();
        number_str.push_str(&caps[1]);
        
        let Some(caps) = first_right.captures(&v_str) else {
            println!("no match!");
            return output;
        };
        number_str.push_str(&caps[1]);
        output.push((number_str).parse::<i32>().unwrap());
    }
    output
}

fn main() {
    let mut file = File::open("input.txt").expect("can't open the file");

    let mut text = String::new();
    file.read_to_string(&mut text).expect("can't read the file");

    let output1 = act1(&text);
    print!("output1: {}\n", output1.iter().copied().sum::<i32>());

    let output2 = act2(&text);
    print!("output2: {}\n", output2.iter().copied().sum::<i32>());
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_example_1() {
        let input="1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let output=vec![12,
        38,
        15,
        77];
        assert_eq!(output, act1(&input.to_string()));
    }


    #[test]
    fn test_example_2() {
        let input="two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let output=vec![29, 83, 13, 24, 42, 14, 76];
        assert_eq!(output, act2(&input.to_string()));
    }
}