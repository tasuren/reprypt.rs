
use std::env;

mod lib;


const HELP: &str = "# Reprypt
Repryptで暗号化します。
## 使用方法
暗号化：reprypt encrypt 内容 パスワード
復号化：reprypt decrypt 暗号文 パスワード";


fn main() {
    let mut args = env::args();
    if args.len() > 1 {
        match (args.nth(1).expect("引数が足りません。")).as_str() {
            "encrypt" => println!(
                "{}", lib::encrypt(
                    args.nth(0).expect("引数が足りません。"),
                    args.nth(0).expect("引数が足りません。"),
                    "base64", true
                ).unwrap_or("Error".to_string())
            ),
            _ => println!("{}", HELP)
        }
    } else {
        println!("{}", HELP);
    }
}