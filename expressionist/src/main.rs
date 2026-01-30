fn main() {
    let speed = 36;
    let time = 3;
    
    let bandwidth = calculate_bandwidth(speed, time);
    println!("The bandwidth is: {bandwidth}");
}

fn calculate_bandwidth(speed: i32, time: i32) -> i32 {
    speed * time
}
