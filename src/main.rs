use hex;
use starknet::core::utils::starknet_keccak;
use starknet_types_core::{
    felt::Felt,
    hash::{Pedersen, StarkHash},
};

fn calculate_message_hash(
    account_address: &str,
    message: &str,
    dapp_name: &str,
    version: &str,
    chain_id: &str,
) -> Felt {
    // 1. Calculate PREFIX
    let starknet_message =
        Felt::from_hex(&format!("0x{}", hex::encode("StarkNet Message".as_bytes()))).unwrap();
    println!("starknet_message: 0x{:x}", starknet_message);

    // 2. Calculate domain_hash
    let domain_type_str = r#""StarkNetDomain"("name":"felt","chainId":"felt","version":"felt")"#;
    let domain_type_hash = starknet_keccak(domain_type_str.as_bytes());
    println!("domain_type_hash: 0x{:x}", domain_type_hash);

    let mut domain_params = vec![domain_type_hash];
    domain_params
        .push(Felt::from_hex(&format!("0x{}", hex::encode(dapp_name.as_bytes()))).unwrap());
    domain_params.push(Felt::from_hex(&format!("0x{}", hex::encode(chain_id.as_bytes()))).unwrap());
    domain_params.push(Felt::from_hex(&format!("0x{}", hex::encode(version.as_bytes()))).unwrap());
    let domain_hash = Pedersen::hash_array(&domain_params);
    println!("domain_hash: 0x{:x}", domain_hash);

    // 3. Calculate account
    let account_address_felt = Felt::from_hex(account_address).unwrap();

    // 4. Calculate message_hash
    let message_type_str = r#""Message"("account_address":"felt","message":"felt")"#;
    let message_type_hash = starknet_keccak(message_type_str.as_bytes());
    println!("message_type_hash: 0x{:x}", message_type_hash);

    let mut message_params = vec![message_type_hash];
    message_params.push(Felt::from_hex(account_address).unwrap());
    message_params.push(Felt::from_hex(&format!("0x{}", hex::encode(message.as_bytes()))).unwrap());
    let message_hash = Pedersen::hash_array(&message_params);
    println!("message_hash: 0x{:x}", message_hash);

    Pedersen::hash_array(&[
        starknet_message,
        domain_hash,
        account_address_felt,
        message_hash,
    ])
}

fn main() {
    let account_address = "0x01b175fe86400121641d32d47490f76cd1ff973a6f090631496c0a08a530ed18";
    let message = "Hello Argent!";
    let dapp_name = "Example DApp";
    let version = "0.0.1";
    let chain_id = "SN_SEPOLIA";

    let message_hash =
        calculate_message_hash(account_address, message, dapp_name, version, chain_id);
    println!("Message hash (decimal): {}", message_hash);
    println!("Message hash (hex): 0x{:x}", message_hash);
}

// TypedData send to Argent:
// TypedData { domain: Domain { name: "Example DApp", version: "0.0.1", chain_id: "SN_SEPOLIA" },
// types: Types { starknet_domain: [Type { name: "name", type_: "felt" }, Type { name: "chainId", type_: "felt" },
// Type { name: "version", type_: "felt" }], message: [Type { name: "account_address", type_: "felt" }, Type { name: "message", type_: "felt" }] },
// primary_type: "Message", message: Message { account_address: "0x01b175fe86400121641d32d47490f76cd1ff973a6f090631496c0a08a530ed18", message: "Hello Argent!" } }

// argent hash of TypedData: 0x124d7017038d208ccef851b8f5465debc38059b2b34cf3ed74ea7cc85ee1c2f
