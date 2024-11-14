use num_bigint::BigInt;

pub trait HashFunction {
    fn hash(&self, a: &BigInt, b: &BigInt) -> BigInt;
}

pub struct SimpleAddHash;

impl HashFunction for SimpleAddHash {
    fn hash(&self, a: &BigInt, b: &BigInt) -> BigInt {
        a + b
    }
}

pub struct CustomHash;

impl HashFunction for CustomHash {
    fn hash(&self, a: &BigInt, b: &BigInt) -> BigInt {
        a + b
    }
}
