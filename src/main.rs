use memguise::get_multi_process_info;
fn main() {
    let info = get_multi_process_info(vec!["firefox.exe", "qbittorrent.exe"]);
    println!("{:?}", info);
}
