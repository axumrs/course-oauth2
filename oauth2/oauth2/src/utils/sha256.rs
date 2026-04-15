use sha2::Digest;

use crate::Result;

pub fn sha256_hash(s: impl AsRef<[u8]>) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(s);

    format!("{:x}", hasher.finalize())
}

pub fn sha256_hash_obj(o: impl serde::Serialize) -> Result<String> {
    let s = serde_json::to_string(&o)?;
    Ok(sha256_hash(s))
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::{sha256_hash as hash, sha256_hash_obj as hash_obj};

    #[test]
    fn test_hash() {
        assert_eq!(
            hash("hello"),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_hash_obj() {
        let h = hash_obj(utils::new_nonce()).unwrap();
        println!("{}", h);
    }
}
