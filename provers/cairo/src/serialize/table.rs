use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use stark_platinum_prover::proof::stark::StarkProof;

use crate::Felt252;
use super::vector::{VectorCommitmentConfigStoneCompatible, VectorCommitmentWitnessStoneCompatible};

#[derive(Debug)]
pub struct TableCommitmentConfigStoneCompatible {
    pub n_columns: Felt252,
    pub vector: VectorCommitmentConfigStoneCompatible,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for TableCommitmentConfigStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct TableDecommitmentStoneCompatible {
    pub n_values: Felt252,
    pub values: Vec<Felt252>,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for TableDecommitmentStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct TableCommitmentWitnessStoneCompatible {
    pub vector: VectorCommitmentWitnessStoneCompatible,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for TableCommitmentWitnessStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}
