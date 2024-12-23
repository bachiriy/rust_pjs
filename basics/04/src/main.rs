use std::{i32, io};

fn main() {
    print!("\n\nHi, this is rustarator\n");
    loop {
        let f_num: i32 = match read_input("first number").trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let opr = read_input("operator");
        let s_num: i32 = match read_input("second number").trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let result: i32 = handle_calc(f_num, opr.trim(), s_num);
        print!("result is {}.\n", result);
        if !wanna_continue() {
            print!("Exiting...\nBye ^^\n");
            break;
        }
    }

}

fn read_input(input_name: &str) -> String {
   print!("please enter {} \n", input_name);
    let mut input = String::from("");
    io::stdin().read_line(&mut input).expect("faild reading input.");
    input
}

fn handle_calc(f_num: i32, opr: &str, s_num: i32) -> i32 {
    if opr.eq("+") {
       f_num + s_num 
    } else if opr.eq("-") {
        f_num - s_num
    } else if opr.eq("/") {
       f_num / s_num 
    } else {
        f_num * s_num 
    }
}

fn wanna_continue() -> bool {
    print!("do you want to do new calculation? (y/n) \n");
    let mut input = String::from("");
    io::stdin().read_line(&mut input).expect("faild reading input.");
    if input.trim().eq("y") || input.trim().eq("Y") {
       true 
    } else {
        false
    }
}
