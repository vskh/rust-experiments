fn tobin(value: u128) -> String {
    let mut converted_bytes: Vec<String> = vec![];
    let mut converted_byte = String::new();

    let mut first_bit = 128;

    for i in (0..128).rev() {
        if value & (1 << i) > 0 {
            if first_bit == 128 {
                first_bit = i;
            }

            converted_byte.push_str("1");
        } else {
            converted_byte.push_str("0");
        }

        if i != 128 && i % 8 == 0 {
            converted_bytes.push(converted_byte);
            converted_byte = String::new();
        }
    }

    converted_bytes
        .into_iter()
        .filter(|byte| byte != "00000000")
        .fold(String::new(), |acc, val| acc + " " + &val)
        .trim()
        .to_owned()
}

fn main() {
    println!("2 = {}", tobin(2));
}
