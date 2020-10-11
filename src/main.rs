use std::env;
use std::process::exit;

fn main() {
    // Confirm the correct number of arguments is present
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { 
        eprintln!("Usage: {} p q e m <OPTION> -d\r\n", &args[0]);
        exit(0);
    }

    // Extract arguments from cli and parse them to u32s
    let p = u32::from_str_radix(&args[1], 10).unwrap();
    let q = u32::from_str_radix(&args[2], 10).unwrap();
    let e = u32::from_str_radix(&args[3], 10).unwrap();

    let message = u32::from_str_radix(&args[4], 10).unwrap();

    // Verify the p != q
    if p == q {
        eprintnln!("P and Q cannot be the same value");
        exit(0);
    }

    // Compute n and phi(n)
    let n = p * q;
    let phi_n = (p - 1) * (q - 1);

    // Verify e is relatively prime to phi_n
    if gcd(phi_n, e) != 1 && e > 1 && e < phi_n {
        eprintln!("E is invalid");
        exit(0);
    }

    let c = sqr_mult(message, e, n);

    if args.len() == 6 && &args[5] == "-d" {
        let d = extended_euclid(phi_n, e).unwrap();
        let m = sqr_mult(message, d, n);
        println!("M: {}", m);
    } else {
        println!("C: {}", c);
    }
}


/// Square multiply algorithm for calculating large powers
///
/// a^b mod n
fn sqr_mult (a: u32, b: u32, n: u32) -> u32 {
    let mut c = 0;
    let mut d = 1;

    // Calculate the binary length of b
    let b_len = (b.count_ones() + b.count_zeros()) - b.leading_zeros();

    // Loop from the length of b in binary down to 0.
    for i in (0..b_len).rev() {
        c = 2 * c;
        d = (d * d) % n;

        // Determine the value of b[i]
        let b_i = (b >> i) & 1;

        if b_i == 1 {
            c += 1;
            d = (d * a) % n;
        }
    }
    
    d
}

fn gcd(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    let mut t: u32;
    loop {
        t = a % b;
        if t == 0 {
            return b;
        }

        a = b;
        b = t;
    }
}

// Euclids extended algorithm for finding the multiplicative inverse.
fn extended_euclid(m: u32, b: u32) -> Option<u32> {
    let mut a1: i32 = 1;
    let mut a2: i32 = 0;
    let mut a3: i32 = m as i32;
    let mut b1: i32 = 0;
    let mut b2: i32 = 1;
    let mut b3: i32 = b as i32;

    loop {
        if b3 == 0 {
            return None;
        }

        if b3 == 1 {
            return Some(b2 as u32);
        }

        let q = a3 / b3;
        let t1 = a1 - (q * b1);
        let t2 = a2 - (q * b2);
        let t3 = a3 - (q * b3);
        a1 = b1;
        a2 = b2;
        a3 = b3;
        b1 = t1;
        b2 = t2;
        b3 = t3;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn square_multiply() {
        assert_eq!(sqr_mult(4, 10, 15), 1);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(20, 45), 5);
    }

}
