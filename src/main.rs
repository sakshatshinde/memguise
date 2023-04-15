use memguise::*;
fn main() {
    let info = get_gpu_memory_info();
    println!("{:?}", info);
}
