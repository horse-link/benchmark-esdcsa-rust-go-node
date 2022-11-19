use std::time::SystemTime;
use ethers;
use ethers::abi::Token;
use ethers::core::types::U256;
use ethers::prelude::{LocalWallet};
use std::thread;
use ethers::types::Signature;

const REPEAT_COUNT: u32 = 1_000_000;
const THREAD_COUNT: u32 = 100;

fn main() {
    let start = SystemTime::now();
    let mut handles = vec![];
    let hashes_per_thread = REPEAT_COUNT / THREAD_COUNT;
    for _ in 0..THREAD_COUNT {
        let handle = thread::spawn(move || {
            for _ in 0..hashes_per_thread {
                hash();
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let duration = SystemTime::now().duration_since(start).unwrap();
    println!("execution took {:#?} milliseconds", duration.as_millis());
}

fn hash() -> Signature {
    // Hash message.
    let hash = ethers::utils::keccak256(ethers::abi::encode(&[
        Token::Uint(U256::from(1337)),
        Token::String("The quick brown fox jumps over the lazy dog".to_string()),
    ]));
    // Sign EIP191 message.
    let signer = get_signer();
    signer.sign_hash(ethers::utils::hash_message(hash))
}

fn get_signer() -> LocalWallet {
    "beefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeef"
        .to_string()
        .parse::<LocalWallet>()
        .unwrap()
}
