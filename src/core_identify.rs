use std::collections::HashMap;

pub fn _identify_hash(){
    let mut hash_points: HashMap<String, i32> = HashMap::new();

    hash_points.insert("MD2".to_string(), 0);
    hash_points.insert("MD4".to_string(), 0);
    hash_points.insert("MD5".to_string(), 0);
    hash_points.insert("LM".to_string(), 0);
    hash_points.insert("NTLM".to_string(), 0);
    hash_points.insert("SHA1".to_string(), 0);
    hash_points.insert("SHA-224".to_string(), 0);
    hash_points.insert("SHA-256".to_string(), 0);
    hash_points.insert("SHA-384".to_string(), 0);
    hash_points.insert("SHA-512".to_string(), 0);
    hash_points.insert("SHA3-224".to_string(), 0);
    hash_points.insert("SHA3-256".to_string(), 0);
    hash_points.insert("SHA3-384".to_string(), 0);
    hash_points.insert("SHA3-512".to_string(), 0);
    hash_points.insert("RIPEMD-128".to_string(), 0);
    hash_points.insert("RIPEMD-160".to_string(), 0);
    hash_points.insert("RIPEMD-256".to_string(), 0);
    hash_points.insert("RIPEMD-320".to_string(), 0);
    hash_points.insert("WHIRLPOOL".to_string(), 0);
    hash_points.insert("TIGER-128".to_string(), 0);
    hash_points.insert("TIGER-160".to_string(), 0);
    hash_points.insert("TIGER-192".to_string(), 0);
    hash_points.insert("BLAKE2s".to_string(), 0);
    hash_points.insert("BLAKE2b".to_string(), 0);
    hash_points.insert("bcrypt".to_string(), 0);
    hash_points.insert("scrypt".to_string(), 0);
    hash_points.insert("PBKDF2".to_string(), 0);
    hash_points.insert("CRC2".to_string(), 0);
    hash_points.insert("Adler32".to_string(), 0);
    hash_points.insert("xxHASH32".to_string(), 0);
    hash_points.insert("xxHASH64".to_string(), 0);
    hash_points.insert("AES".to_string(), 0);
    hash_points.insert("DES".to_string(), 0);
    hash_points.insert("3DES".to_string(), 0);
    hash_points.insert("Blowfish".to_string(), 0);
    hash_points.insert("3DES".to_string(), 0);

}