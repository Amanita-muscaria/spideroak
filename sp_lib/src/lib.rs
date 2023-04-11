const ALPHABET_LEN: u8 = 25;
const ASCII_LOW: u8 = 97;
const ASCII_UP: u8 = 65;

//Encrypt a message using a simple Caesar cipher
pub fn encrypt(mut key: u8, msg: String) -> String {
    key = key % ALPHABET_LEN;

    msg.bytes()
        .into_iter()
        .map(|c| match c as char {
            'a'..='z' => {
                let mut e = c - ASCII_LOW;
                e += key;
                e = e % (ALPHABET_LEN + 1);
                e + ASCII_LOW
            }
            'A'..='Z' => {
                let mut e = c - ASCII_UP;
                e += key;
                e = e % (ALPHABET_LEN + 1);
                e + ASCII_UP
            }
            _ => c,
        } as char)
        .collect()
}

//Decrypt a message using a simple Caesar cipher
pub fn decrypt(mut key: u8, msg: String) -> String {
    key = key % ALPHABET_LEN;
    msg.bytes()
        .into_iter()
        .map(|c| {
            return match c as char {
                'a'..='z' => {
                    let mut e = c - ASCII_LOW;
                    e = e
                        .checked_sub(key)
                        .unwrap_or_else(|| ALPHABET_LEN + 1 + e - key);
                    e + ASCII_LOW
                }
                'A'..='Z' => {
                    let mut e = c - ASCII_UP;
                    e = e
                        .checked_sub(key)
                        .unwrap_or_else(|| ALPHABET_LEN + 1 + e - key);
                    e + ASCII_UP
                }
                _ => c,
            } as char;
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_encrypt_test() {
        let msg: String = String::from("abc");
        let key: u8 = 1;

        assert_eq!(encrypt(key, msg), String::from("bcd"))
    }

    #[test]
    fn simple_decrypt_test() {
        let msg: String = String::from("bcd");
        let key: u8 = 1;

        assert_eq!(decrypt(key, msg), String::from("abc"))
    }

    #[test]
    fn encrypt_symbols_test() {
        let msg: String = String::from("b cd.");
        let key: u8 = 1;

        assert_eq!(encrypt(key, msg), String::from("c de."))
    }

    #[test]
    fn decrypt_symbols_test() {
        let msg: String = String::from("cd e.");
        let key: u8 = 1;

        assert_eq!(decrypt(key, msg), String::from("bc d."))
    }

    #[test]
    fn encrypt_numbers_test() {
        let msg: String = String::from("cde123");
        let key: u8 = 1;

        assert_eq!(encrypt(key, msg), String::from("def123"))
    }

    #[test]
    fn decrypt_numbers_test() {
        let msg: String = String::from("def123");
        let key: u8 = 1;

        assert_eq!(decrypt(key, msg), String::from("cde123"))
    }

    #[test]
    fn encrypt_wrap_test() {
        let msg: String = String::from("xyz");
        let key: u8 = 2;

        assert_eq!(encrypt(key, msg), String::from("zab"))
    }

    #[test]
    fn decrypt_wrap_test() {
        let msg: String = String::from("zab");
        let key: u8 = 2;
        assert_eq!(decrypt(key, msg), String::from("xyz"))
    }

    #[test]
    fn encrypt_combo_test() {
        let msg: String = String::from("xYz.123");
        let key: u8 = 2;

        assert_eq!(encrypt(key, msg), String::from("zAb.123"))
    }

    #[test]
    fn decrypt_combo_test() {
        let msg: String = String::from("zAb.123");
        let key: u8 = 2;

        assert_eq!(decrypt(key, msg), String::from("xYz.123"))
    }

    #[test]
    fn big_key_test() {
        let msg: String = String::from("abc");
        let key: u8 = ALPHABET_LEN + 1;

        assert_eq!(encrypt(key, msg), String::from("bcd"))
    }
}
