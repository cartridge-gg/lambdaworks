use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use stark_platinum_prover::proof::stark::StarkProof;

use crate::Felt252;

#[derive(Debug)]
pub struct VectorCommitmentConfigStoneCompatible {
    pub height: Felt252,
    pub n_verifier_friendly_commitment_layers: Felt252,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for VectorCommitmentConfigStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct VectorCommitmentWitnessStoneCompatible {
    pub n_authentications: Felt252,
    pub authentications: Vec<Felt252>,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for VectorCommitmentWitnessStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}
