// fn main() {
//     another_function('h');
// }
// 
// fn another_function(x: char) {
//     println!("The value of x is: {x}");
// }
fn main() {
    let y = {
        let x = 10;
        let x = x + 1;
        x + 2
    };

    fn greet_sm1(name: &str) -> &str {
        "Hi " + name
    }
    let rslt: &str = greet_sm1("bsh");
    println!("{}", rslt);
}
