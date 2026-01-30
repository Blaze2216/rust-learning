fn main() {
    println!("Hello this is main funtion.");

    another_function(5, 'a');
    
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
    
    let num = five();
    println!("The value of num is: {num}");
    
    let num2 = plus_one(5);
    println!("The value of num2 is: {num2}");
}
// Parameters pass name: type
fn another_function(value: u8, char_value: char) {
    println!("The value is: {value}");
    println!("The of char is: {char_value}");
} 

fn five() -> i32 /* This is the return type -> i32 */ {
    5
}

fn plus_one(num2: i32) -> i32 {
    num2 + 1
}