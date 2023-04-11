//Encrypt a message using a simple Caesar cipher
pub fn encrypt(key: usize, msg: String) -> String {
    todo!();
}

//Decrypt a message using a simple Caesar cipher
pub fn decrypt(key: usize, msg: String) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_encrypt_test() {
        let msg: String = String::from("abc");
        let key: usize = 1;

        assert_eq!(encrypt(key, msg), String::from("bcd"))
    }

    #[test]
    fn simple_decrypt_test() {
        let msg: String = String::from("bcd");
        let key: usize = 1;

        assert_eq!(decrypt(key, msg), String::from("abc"))
    }

    #[test]
    fn encrypt_symbols_test() {
        let msg: String = String::from("b cd.");
        let key: usize = 1;

        assert_eq!(encrypt(key, msg), String::from("c de."))
    }

    #[test]
    fn decrypt_symbols_test() {
        let msg: String = String::from("cd e.");
        let key: usize = 1;

        assert_eq!(decrypt(key, msg), String::from("bc d."))
    }

    #[test]
    fn encrypt_numbers_test() {
        let msg: String = String::from("cde123");
        let key: usize = 1;

        assert_eq!(encrypt(key, msg), String::from("def123"))
    }

    #[test]
    fn decrypt_numbers_test() {
        let msg: String = String::from("def123");
        let key: usize = 1;

        assert_eq!(decrypt(key, msg), String::from("cde123"))
    }

    #[test]
    fn encrypt_wrap_test() {
        let msg: String = String::from("xyz");
        let key: usize = 2;

        assert_eq!(encrypt(key, msg), String::from("zab"))
    }

    #[test]
    fn decrypt_wrap_test() {
        let msg: String = String::from("zab");
        let key: usize = 2;

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
}
