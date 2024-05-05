use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

fn main() {
    let key = GenericArray::from([0u8; 16]);
    let mut block = GenericArray::from([42u8; 16]);

    // Initialize cipher
    let cipher = Aes128::new(&key);

    let block_copy = block.clone();

    // Encrypt block in-place
    cipher.encrypt_block(&mut block);

    

    // And decrypt it back
    cipher.decrypt_block(&mut block);
    assert_eq!(block, block_copy);

    // Implementation supports parallel block processing. Number of blocks
    // processed in parallel depends in general on hardware capabilities.
    // This is achieved by instruction-level parallelism (ILP) on a single
    // CPU core, which is differen from multi-threaded parallelism.
    let mut blocks = [block; 100];
    cipher.encrypt_blocks(&mut blocks);

    for block in blocks.iter_mut() {
        cipher.decrypt_block(block);
        assert_eq!(block, &block_copy);
    }

    // `decrypt_blocks` also supports parallel block processing.
    cipher.decrypt_blocks(&mut blocks);

    for block in blocks.iter_mut() {
        cipher.encrypt_block(block);
        assert_eq!(block, &block_copy);
    }
}
