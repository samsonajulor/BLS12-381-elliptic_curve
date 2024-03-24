use std::env;
use dotenv::dotenv;
use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;

fn main() {
    dotenv().ok();

    let secret_key_str = env::var("SECRET_KEY").expect("SECRET_KEY environment variable not set");
    let secret_key: u64 = u64::from_str_radix(&secret_key_str, 16)
        .expect("Failed to parse secret key as hexadecimal");

    let g = BLS12381Curve::generator();
    let public_key = g.operate_with_self(secret_key);
    
    println!("Secret key: {:?}", secret_key);
    println!("Public key: {:?}", public_key);
    let public_key_affine = public_key.to_affine();

    let x = public_key_affine.x();
    let y = public_key_affine.y();

    println!("Public key (x): {}", x);
    println!("Public key (y): {}", y);
}
