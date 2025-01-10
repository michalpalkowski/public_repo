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

#[derive(Debug)]
struct StarkNetDomain {
    name: Felt,
    version: Felt,
    chain_id: Felt,
}

#[derive(Debug)]
struct Message {
    account_address: Felt,
    message: Felt,
}

fn calculate_message_hash(
    account_address: &str,
    message: &str,
    dapp_name: &str,
    version: &str,
    chain_id: &str,
) -> Felt {
    // Domain separator calculation
    let domain = StarkNetDomain {
        name: encode_string(dapp_name),
        version: encode_string(version),
        chain_id: encode_string(chain_id),
    };

    let domain_type_hash =
        starknet_keccak("StarkNetDomain(name:felt,version:felt,chainId:felt)".as_bytes());
    let domain_struct = vec![
        Felt::from(domain_type_hash),
        domain.name,
        domain.version,
        domain.chain_id,
    ];
    let domain_hash = Pedersen::hash_array(&domain_struct);
    println!("Domain hash: 0x{:x}", domain_hash);

    // Message hash calculation
    let msg = Message {
        account_address: encode_string(account_address),
        message: encode_string(message),
    };

    let message_type_hash =
        starknet_keccak("Message(account_address:felt,message:felt)".as_bytes());
    let message_struct = vec![
        Felt::from(message_type_hash),
        msg.account_address,
        msg.message,
    ];
    let message_hash = Pedersen::hash_array(&message_struct);
    println!("Message hash: 0x{:x}", message_hash);

    // Final hash calculation
    Pedersen::hash_array(&[
        encode_string("StarkNet Message"),
        domain_hash,
        msg.account_address,
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

    let message_hash =
        calculate_message_hash(account_address, message, dapp_name, version, chain_id);
    println!("\nFinal hash: 0x{:x}", message_hash);
}
