
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
        let (mode, text, key) = (
            args.nth(1).expect("引数が足りません。"),
            args.nth(0).expect("引数が足りません。"),
            args.nth(0).expect("引数が足りません。")
        );
        match mode.as_str() {
            "encrypt" => println!(
                "{}", lib::encrypt(text, key, "base64", true)
            ),
            "decrypt" => println!(
                "{}", lib::decrypt(text, key, "base64", true).unwrap_or(
                    "エラーが発生しました。パスワードがあっているか確認してください。"
                    .to_string()
                )
            ),
            _ => println!("{}", HELP)
        }
    } else {
        println!("{}", HELP);
    }
}