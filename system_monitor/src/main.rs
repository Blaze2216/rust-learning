fn main() {
    let gpu_status: (&str, f32, bool) = ("NVIDIA", 43.5, true);
    
    let name = gpu_status.0;
    let temp = gpu_status.1;
    let fan_active = gpu_status.2;
    
    println!("GPU name: {name}");
    println!("Temperature : {temp}");
    println!("Fan Active: {fan_active}");
}
