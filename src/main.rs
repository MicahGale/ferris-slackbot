use std::process::Command;

fn main() {
    let version = get_compiler_version();
    let binary_version = convert_text_to_binary(&version);
}

fn get_compiler_version() -> String {
    let output = Command::new("rustc")
        .arg("-V")
        .output()
        .expect("Failed command")
        .stdout;
    String::from_utf8(output).expect("broken")
}

fn convert_text_to_binary(message: &String) -> String {
    let nums = message.clone().into_bytes();
    let mut ret: String = "".to_owned();
    for num in nums {
        ret.push_str(&format!("{:0>8b}", num));
    }
    ret = ret.replace("1", "🦀").replace("0", "🦝");
    ret
}
