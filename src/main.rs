use hex;
use starknet::core::utils::starknet_keccak;
use starknet_types_core::{
    felt::Felt,
    hash::{Pedersen, StarkHash},
};

fn encode_string(s: &str) -> Felt {
    if s.starts_with("0x") {
        Felt::from_hex(s).unwrap()
    } else {
        let encoded = hex::encode(s.as_bytes());
        Felt::from_hex(&format!("0x{}", encoded)).unwrap()
    }
}

fn calculate_message_hash(
    account_address: &str,
    message: &str,
    dapp_name: &str,
    version: &str,
    chain_id: &str,
) -> Felt {
    // Calculate domain separator
    let domain_type_str = r#""StarkNetDomain"("name":"felt","chainId":"felt","version":"felt")"#;
    let domain_type_hash = starknet_keccak(domain_type_str.as_bytes());
    println!("Domain type hash: 0x{:x}", domain_type_hash);

    let domain_struct = vec![
        domain_type_hash,
        encode_string(dapp_name),
        encode_string(version),
        encode_string(chain_id),
    ];
    
    let domain_hash = Pedersen::hash_array(&domain_struct);
    println!("Domain hash: 0x{:x}", domain_hash);

    // Calculate message hash
    let message_type_str = r#""Message"("account_address":"felt","message":"felt")"#;
    let message_type_hash = starknet_keccak(message_type_str.as_bytes());
    println!("Message type hash: 0x{:x}", message_type_hash);

    let message_struct = vec![
        message_type_hash,
        encode_string(account_address),
        encode_string(message),
    ];
    
    let message_hash = Pedersen::hash_array(&message_struct);
    println!("Message hash: 0x{:x}", message_hash);

    // Calculate final hash
    
    Pedersen::hash_array(&[
        encode_string("StarkNet Message"),
        domain_hash,
        encode_string(account_address),
        message_hash,
    ])
}

fn main() {
    let account_address = "0x01b175fe86400121641d32d47490f76cd1ff973a6f090631496c0a08a530ed18";
    let message = "Hello Argent!";
    let dapp_name = "Example DApp";
    let version = "0.0.1";
    let chain_id = "SN_SEPOLIA";

    println!("\nInput values:");
    println!("Account: {}", account_address);
    println!("Message: {}", message);
    println!("DApp name: {}", dapp_name);
    println!("Version: {}", version);
    println!("Chain ID: {}", chain_id);
    println!("\nHashing steps:");

    let message_hash = calculate_message_hash(account_address, message, dapp_name, version, chain_id);
    println!("\nFinal hash: 0x{:x}", message_hash);
}