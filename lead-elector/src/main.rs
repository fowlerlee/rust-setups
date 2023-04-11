use rand::Rng;
use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, Zero};

// Diffie-Hellman key exchange parameters
const PRIME_SIZE: usize = 2048; // Size of the prime number in bits
const BASE: u64 = 2; // Base value

// Step 1: Generate random prime numbers for public and private keys
fn generate_keys() -> (BigUint, BigUint, BigUint) {
    let mut rng = rand::thread_rng();

    // Generate random prime number for modulus
    let prime = rng.gen_prime(PRIME_SIZE);

    // Generate random private keys for Alice and Bob
    let alice_private = rng.gen_biguint(PRIME_SIZE);
    let bob_private = rng.gen_biguint(PRIME_SIZE);

    // Calculate public keys for Alice and Bob
    let alice_public = mod_exp(BASE, &alice_private, &prime);
    let bob_public = mod_exp(BASE, &bob_private, &prime);

    // Return the prime number and public/private keys for Alice and Bob
    (prime, alice_public, bob_public)
}

// Step 2: Compute the shared secret key
fn compute_shared_secret_key(
    my_private: &BigUint,
    other_public: &BigUint,
    prime: &BigUint,
) -> BigUint {
    // Compute the shared secret key using the received public key and my private key
    mod_exp(other_public, my_private, prime)
}

// Step 3: Modular exponentiation function
fn mod_exp(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    let one = BigUint::one();
    let zero = BigUint::zero();

    // If exponent is zero, return 1
    if *exponent == zero {
        return one;
    }

    // If exponent is odd, compute (base ^ exponent) mod modulus
    if exponent.is_odd() {
        let result = mod_exp(base, &(exponent - &one), modulus);
        return (base * &result) % modulus;
    }

    // If exponent is even, compute ((base ^ (exponent / 2)) ^ 2) mod modulus
    let result = mod_exp(base, &(exponent >> 1), modulus);
    (result * &result) % modulus
}

fn main() {
    // Step 1: Generate keys for Alice and Bob
    let (prime, alice_public, bob_public) = generate_keys();

    println!("Prime: {}", prime);
    println!("Alice's public key: {}", alice_public);
    println!("Bob's public key: {}", bob_public);

    // Step 2: Compute shared secret keys for Alice and Bob
    let alice_private = rand::thread_rng().gen_biguint(PRIME_SIZE);
    let alice_shared_secret = compute_shared_secret_key(&alice_private, &bob_public, &prime);

    let bob_private = rand::thread_rng().gen_biguint(PRIME_SIZE);
    let bob_shared_secret = compute_shared_secret_key(&bob_private, &alice_public, &prime);

    // Step 3: Verify that Alice and Bob's shared secret keys match
    assert_eq!(alice_shared_secret, bob_shared_secret);

    println!("Shared secret key: {}", alice_shared_secret);
}
