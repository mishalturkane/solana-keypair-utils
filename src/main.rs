mod keypair;

use dotenv::dotenv;
use std::env;

use keypair::base58_to_json::base58_to_json;
use keypair::json_to_base58::json_to_base58;
use keypair::search_key::search_key;

fn main() {
    // .env load the env
    dotenv().ok();

    let private_key = env::var("PRIVATE_KEY_BASE58")
        .expect("❌ PRIVATE_KEY_BASE58 not found in .env!");

    let vanity_prefix = env::var("VANITY_PREFIX")
        .expect("❌ VANITY_PREFIX  not found in .env!");

    let vanity_output = env::var("VANITY_OUTPUT_FILE")
        .expect("❌ VANITY_OUTPUT_FILE not found in .env!");

    let input_json = env::var("INPUT_JSON_FILE")
        .expect("❌ INPUT_JSON_FILE not found in .env!"); 

    println!("\n--- 1️⃣  Base58 → JSON ---");
    base58_to_json(&private_key, &input_json);

    println!("\n--- 2️⃣  JSON → Base58 ---");
    json_to_base58(&input_json);

    println!("\n--- 3️⃣  Vanity Address Search ---");
    search_key(&vanity_prefix, &vanity_output);
}