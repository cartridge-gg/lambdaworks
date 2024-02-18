use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use stark_platinum_prover::proof::stark::StarkProof;

use crate::Felt252;
use super::table::{TableCommitmentConfigStoneCompatible, TableDecommitmentStoneCompatible, TableCommitmentWitnessStoneCompatible};

#[derive(Debug)]
pub struct TracesConfigStoneCompatible {
    pub original: TableCommitmentConfigStoneCompatible,
    pub interaction: TableCommitmentConfigStoneCompatible,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for TracesConfigStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct TracesDecommitmentStoneCompatible {
    pub original: TableDecommitmentStoneCompatible,
    pub interaction: TableDecommitmentStoneCompatible,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for TracesDecommitmentStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct TracesUnsentCommitmentStoneCompatible {
    pub original: Felt252,
    pub interaction: Felt252,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for TracesUnsentCommitmentStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct TracesWitnessStoneCompatible {
    pub original: TableCommitmentWitnessStoneCompatible,
    pub interaction: TableCommitmentWitnessStoneCompatible,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for TracesWitnessStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}
