use rand::Rng;

fn main() {
    // Generate two small prime numbers
    let p = generate_prime(10);
    let q = generate_prime(10);

    // Calculate n (the modulus)
    let n = p * q;

    // Calculate Euler's Totient function, phi(n)
    let phi = (p - 1) * (q - 1);

    // Choose a small public exponent, e
    let e = 3; // Typically, 3 or 65537 are used in practice

    // Calculate the private exponent, d
    let d = mod_inverse(e, phi);

    // Public key: (n, e)
    // Private key: (n, d)

    println!("Public key (n, e): ({}, {})", n, e);
    println!("Private key (n, d): ({}, {})", n, d);
}

fn generate_prime(bits: usize) -> u64 {
    let mut rng = rand::thread_rng();
    loop {
        let candidate = rng.gen_range(2u64.pow(bits as u32 - 1)..2u64.pow(bits as u32));
        if is_prime(candidate) {
            return candidate;
        }
    }
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..((n as f64).sqrt() as u64 + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn mod_inverse(a: u64, m: u64) -> u64 {
    for x in 1..m {
        if (a * x) % m == 1 {
            return x;
        }
    }
    panic!("Modular inverse does not exist");
}
