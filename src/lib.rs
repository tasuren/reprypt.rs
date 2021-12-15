//! # Reprypt
//! Repryptという暗号化モジュールです。


/// 指定されたモードでデコードまたは変換を行う。
/// 例：base64
fn decode(text: String, mode: &str) -> Option<String> {
    match mode {
        "base64" => {
            match base64::decode(text) {
                Ok(v) => String::from_utf8(v).ok(),
                _ => None,
            }
        },
        _ => panic!("That mode is not supported.")
    }
}


/// 指定されたモードでエンコードまたは変換を行う。
fn encode(text: String, mode: &str) -> String {
    match mode {
        "base64" => base64::encode(text),
        _ => panic!("That mode is not supported.")
    }
}


/// 渡された文字列の指定されたインデックスにある文字を取り出す。
fn get_byindex(text: &String, index: usize, length: Option<usize>) -> String {
    (if index < length.unwrap_or_else(|| text.len()) {
        &text[index..index + 1]
    } else {
        &text[index - 1..]
    }).to_string()
}


/// Unicodeポイントに変換する。
fn convert_unicode(text: &String) -> String {
    // debug: println!("Raw Key: {}", &text);
    let mut new = String::new();
    for char in text.as_str().chars() {
        new += &((char as u32).to_string());
    };
    // debug: println!("Raw Key: {}", &new);
    new
}


/// パスワードを適切な形に整える。
fn parse_key(mut key: String, mut key_length: usize, text_length: usize) -> String {
    let mut error: usize;
    while key_length < text_length {
        error = text_length - key_length;
        if error > key_length {
            error -= error - key_length;
        };
        key = key.to_string() + &key[key_length - error..];
        key_length += error;
    }
    (&key[..text_length]).to_string()
}


/// 渡された文字列の特定の位置にある文字を特定の位置に置き換える関数です。
fn replace(mut text: String, length: usize, from: usize, to: usize) -> String {
    // debug: println!("{} {} {}", text, from, to);
    let after = get_byindex(&text, to, Some(length));
    let end = to + 1;
    let end = if end < length {
        (&text[end..]).to_string()
    } else {
        "".to_string()
    };
    text = (&text[..to]).to_string() + &get_byindex(&text, from, Some(length)) + &end;
    let end = from + 1;
    let end = if end < length {
        (&text[end..]).to_string()
    } else {
        "".to_string()
    };
    (&text[..from]).to_string() + &after + &end
}


/// 渡された文字列を暗号化します。
pub fn encrypt(
    mut text: String, _key: String, mode: &str, convert: bool
) -> String {
    // デコードをするべきならデコードをする。(強度向上のため。)
    if convert {
        text = encode(text, mode);
    }

    let mut key_index: usize = 0;
    let text_length = text.len();
    let mut key = convert_unicode(&_key);
    let key_length = (&key).len();
    key = parse_key(key, key_length, text_length);
    // 暗号化する。
    let mut target: usize;
    for index in 0..text.len() {
        key_index += 1;
        target = get_byindex(
            &key, key_index - 1, Some(text_length)
        ).parse().unwrap();
        if target >= text_length {
            target = (target / 2) as usize;
        };
        text = replace(text, text_length, index, target);
    };
    text
}


/// 渡された文字列を復号化します。
pub fn decrypt(
    mut text: String, _key: String, mode: &str, convert: bool
) -> Option<String> {
    let text_length = text.len();
    let mut key = convert_unicode(&_key);
    let key_length = (&key).len();
    key = parse_key(key, key_length, text_length);
    let mut key_index = text_length;
    let mut target: usize;
    // 復号化する。
    for index in (0..text_length).rev() {
        key_index -= 1;
        target = get_byindex(
            &key, key_index, Some(text_length)
        ).parse().unwrap();
        if target >= text_length {
            target = (target / 2) as usize;
        }
        text = replace(text, text_length, target, index);
    };
    // もしデコードするべきならデコードを行う。
    let new_text = if convert { decode(text, mode) } else { Some(text) };
    new_text
}