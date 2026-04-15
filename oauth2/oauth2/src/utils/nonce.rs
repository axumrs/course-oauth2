const DEFAULT_SIZE: usize = 32;

pub fn new_nonce() -> Vec<u8> {
    nonce_with_size(DEFAULT_SIZE)
}

pub fn nonce_with_size(size: usize) -> Vec<u8> {
    rand::random_iter().take(size).collect()
}
