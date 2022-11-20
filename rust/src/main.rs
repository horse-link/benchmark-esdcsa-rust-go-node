use std::str::FromStr;
use std::time::SystemTime;
use ethers;
use ethers::abi::Token;
use ethers::core::types::U256;
use ethers::prelude::{LocalWallet};
use std::thread;
use ethers::types::{Address, Signature};
use ethers::types::transaction::eip1559::Eip1559TransactionRequest;
use ethers::types::transaction::eip2718::TypedTransaction;
use rayon::prelude::*;

const REPEAT_COUNT: usize = 100_000;

fn main() {
    let start = SystemTime::now();
    let signer = get_signer();
    let _: Vec<Signature>  = vec![0; REPEAT_COUNT]
        .par_iter()
        .map(|_| hash(&signer))
        .collect();
    let duration = SystemTime::now().duration_since(start).unwrap();
    println!("{:#?}", duration.as_millis());
}

fn hash(signer: &LocalWallet) -> Signature {
    // Hash message.
    let hash = ethers::utils::keccak256(ethers::abi::encode(&[
        Token::String("The quick brown fox jumps over the lazy dog".to_string()),
        Token::Uint(U256::from(1337)),
    ]));
    // Sign EIP191 message.
    let signature = signer.sign_hash(ethers::utils::hash_message(hash));
    // Build the tx.
    let address = Address::from_str("0xeC9D1C79A92A6c108b6aa9B101E86d58034843B5").unwrap();
    let tx = Eip1559TransactionRequest::new()
        .from(address.clone())
        .to(address)
        .value(U256::from(0))
        .data(signature.to_vec());
    signer.sign_transaction_sync(&TypedTransaction::Eip1559(tx))
}

fn get_signer() -> LocalWallet {
    "beefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeefbeef"
        .to_string()
        .parse::<LocalWallet>()
        .unwrap()
}
