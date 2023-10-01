use md5;

fn main() {
    let key = b"yzbqklnj";

    // Part 1 & 2
    for i in 0..=1 {
        let mut count: u32 = 0;
        loop {
            let input = [key, count.to_string().as_bytes()].concat();
            let digest = format!("{:x}", md5::compute(input.as_slice()));

            if digest.starts_with("0".repeat(5 + i).as_str()) {
                println!("P{}: {} (count: {})", i + 1, digest, count);
                break;
            }

            count += 1;
        }
    }
}
