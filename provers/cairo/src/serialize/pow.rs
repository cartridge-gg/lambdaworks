use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use stark_platinum_prover::proof::stark::StarkProof;

use crate::Felt252;

#[derive(Debug)]
pub struct ProofOfWorkConfigStoneCompatible {
    // Proof of work difficulty (number of bits required to be 0).
    pub n_bits: Felt252,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for ProofOfWorkConfigStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct ProofOfWorkUnsentCommitmentStoneCompatible {
    pub nonce: Felt252,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for ProofOfWorkUnsentCommitmentStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}
