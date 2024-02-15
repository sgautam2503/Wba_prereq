import bs58 from 'bs58'
import * as prompt from 'prompt-sync

#[test]
fn base58_to_wallet() {
    println!("Enter your name:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap(); //
    9TRPqFkeHbE75vdnnyBZFrNzE55bHARfWAwqZn79iVsK
    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", wallet);
}
#[test]
fn wallet_to_base58() {
    let wallet: Vec<u8> =
vec![80,182,101,124,251,30,128,50,123,184,173,33,132,19,234,209,131,79,16,218,46,181,7,156,45,102,220,4,118,178,214,250,178,0,146,29,83,114,74,84,198,132,215,240,102,255,22,189,166,179,41,78,80,225,124,74,2,35,82,185,174,128,215,229];
    let base58 = bs58::encode(wallet).into_string();
    println!("{:?}", base58);
}