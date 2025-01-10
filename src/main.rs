use hex;
use starknet::core::utils::starknet_keccak;
use starknet_crypto::verify;
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
    nonce: Felt,
    issued_at: Felt,
    temporary_public_key: Felt,
}

fn calculate_message_hash(
    account_address: &str,
    message: &str,
    dapp_name: &str,
    version: &str,
    chain_id: &str,
    nonce: &str,
    issued_at: &str,
    temporary_public_key: &str,
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
        nonce: encode_string(nonce),
        issued_at: encode_string(issued_at),
        temporary_public_key: encode_string(temporary_public_key),
    };

    let message_type_hash =
        starknet_keccak(
            "Message(account_address:felt,message:felt,nonce:felt,issued_at:felt,temporary_public_key:felt)".as_bytes()
        );
    let message_struct = vec![
        Felt::from(message_type_hash),
        msg.account_address,
        msg.message,
        msg.nonce,
        msg.issued_at,
        msg.temporary_public_key,
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
    let nonce = "JLFX1HWTq9AkOUQj6";
    let issued_at = "2025-01-10T08:48:36.063Z";
    let temporary_public_key = "0x5b48dcbd15b8eab614c8df5ceccbfdaf111b0f15118d4089ea735c0ebcc2613";

    println!("\nInput values:");
    println!("Account: {}", account_address);
    println!("Message: {}", message);
    println!("DApp name: {}", dapp_name);
    println!("Version: {}", version);
    println!("Chain ID: {}", chain_id);
    println!("Nonce: {}", nonce);
    println!("Issued At: {}", issued_at);
    println!("Temporary Public Key: {}", temporary_public_key);
    println!("\nHashing steps:");

    let message_hash = calculate_message_hash(
        account_address,
        message,
        dapp_name,
        version,
        chain_id,
        nonce,
        issued_at,
        temporary_public_key,
    );
    println!("\nFinal hash: 0x{:x}", message_hash);

    let public_key =
        Felt::from_hex("0x3f7b2d0ed3d5eca3e0e2a80dfbdd60f981fa6faee2a4f6995980362f7dd4793")
            .unwrap();
    let r = Felt::from_hex("0x6e73b7b094ca892a5f68be0f2fb6e4fa6098e364d4112a215994599105fd665")
        .unwrap();
    let s = Felt::from_hex("0x799216b1ac1cb79a6ce3e311a0086d0d7f6ad005b4c41399425df0c1d9983ac")
        .unwrap();

    let is_valid = verify(&public_key, &message_hash, &r, &s);
    println!("\nECDSA signature verification result: {:?}", is_valid);
}
