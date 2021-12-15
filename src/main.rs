//! Repryptのコンソールアプリ用のプログラム

use std::env;
mod lib;


const HELP: &str = "# Reprypt v0.1.1
Repryptで暗号化します。
## 使用方法
暗号化：reprypt encrypt 内容 パスワード
復号化：reprypt decrypt 暗号文 パスワード";
const NONARG: &str = "Error: 引数が足りません。";


fn main() {
    let mut args = env::args();
    if args.len() > 1 {
        let (mode, text, key) = (
            args.nth(1).expect(NONARG),
            args.nth(0).expect(NONARG),
            args.nth(0).expect(NONARG)
        );
        match mode.as_str() {
            "encrypt" => println!(
                "{}", lib::encrypt(text, key, true)
            ),
            "decrypt" => println!(
                "{}", lib::decrypt(text, key, true).unwrap_or(
                    "Error: 複合化に失敗しました。パスワードがあっているか確認してください。"
                    .to_string()
                )
            ),
            _ => println!("{}", HELP)
        }
    } else {
        println!("{}", HELP);
    }
}