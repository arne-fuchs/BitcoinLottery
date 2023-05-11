use std::arch::x86_64::_CMP_UNORD_Q;
use std::collections::HashMap;
use std::env::args;
use std::fs;
use std::fs::{File, OpenOptions, read};
use std::hash::Hash;
use std::io::{BufRead, BufReader, Bytes, Read};
use std::ops::{Index, Range};
use std::str::FromStr;
use std::time::{Duration, Instant};

use bitcoin::{Address, PublicKey, secp256k1};
use bitcoin::bip32::{ChildNumber, DerivationPath, ExtendedPrivKey, ExtendedPubKey};
use bitcoin::hashes::hex::FromHex;
use bitcoin::secp256k1::Secp256k1;
use hex::FromHex as OtherFromHex;
use rand::Rng;
use reqwest::{Error, Response};
use reqwest::header::DATE;
use secp256k1::ffi::types::AlignedType;

fn main() {
    println!("Opening UTXO Dump CSV...");
    let utxos = File::open("utxodump.csv").unwrap();
    println!("Done!");
    println!("Importing UTXO's...");
    let mut reader = BufReader::new(utxos);
    let mut line = String::new();
    let mut utxo_hash_map : HashMap<String,String> = HashMap::new();
    let _ = reader.read_line(&mut line);
    line.clear();

    for line in reader.lines() {
        let line = line.unwrap();
        let splits: Vec<&str> = line.split(",").collect();
        let amount = splits.get(3).unwrap();
        let address = splits.get(5).unwrap();
        utxo_hash_map.insert(address.parse().unwrap(), amount.parse().unwrap());
    }

    println!("Done");

    let mut now = Instant::now();
    let mut hash_count = 0;

    let network = bitcoin::Network::Bitcoin;
    loop {
        //println!("------------------------------------------------------------------------------");

        let mut generator = rand::thread_rng();
        let mut seed: Vec<u8> = vec![];
        for _i in 0..32 {
            let byte_range = 0..=255;
            seed.push(generator.gen_range(byte_range))
        }
        //let mut seed_hex = "b67f4c4c0ac9d078ebd4d4e3255bfe67c2dcd0e5e5d5c5204c833f9f5d5faeb4".to_string();
        //let mut hex = hex::decode(seed_hex.clone()).unwrap();

        //println!("Seed Hex: {}", seed_hex);
        let mut buf: Vec<AlignedType> = Vec::new();
        buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
        let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

        let root = ExtendedPrivKey::new_master(network, &seed).unwrap();

        // derive child xpub
        let path = DerivationPath::from_str("m/84h/0h/0h").unwrap();
        let child = root.derive_priv(&secp, &path).unwrap();
        let xpub = ExtendedPubKey::from_priv(&secp, &child);
        //println!("Public key at {}: {}", path, xpub);

        // generate first receiving address at m/0/0
        // manually creating indexes this time
        let zero = ChildNumber::from_normal_idx(0).unwrap();
        let public_key = xpub.derive_pub(&secp, &[zero, zero]).unwrap().public_key;
        let address = Address::p2wpkh(&PublicKey::new(public_key), network).unwrap();
        let address_str = address.to_string();
        //println!("First receiving address: {}", address_str);

        match utxo_hash_map.get(address_str.as_str()) {
            None => {}
            Some(value) => {
                let seed_hex = hex::encode(seed).to_string();
                fs::write(seed_hex.clone(), format!("Seed: {}\nAddress: {}\nBalance: {}", seed_hex.clone(), address.to_string(), value)).unwrap();
                println!("FOUND KEY: {} VALUE: {} ADDRESS: {}",seed_hex.clone(),value.clone(),address_str.clone());
            }
        }
        hash_count = hash_count + 1;
        let elapsed_time = now.elapsed();
        if elapsed_time >= Duration::from_secs(10) {
            println!("{} Hs/s",hash_count/10);
            hash_count = 0;
            now = Instant::now();
        }
    }
}
