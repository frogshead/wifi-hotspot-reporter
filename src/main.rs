use wifiscanner;
fn main() {
    let wifis = wifiscanner::scan().unwrap();
    for wifi in wifis{
        println!("{:?}", wifi.ssid);
    }
}
