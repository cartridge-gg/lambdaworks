use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use stark_platinum_prover::proof::stark::StarkProof;

use super::{
    vector::{VectorCommitmentConfigStoneCompatible, VectorCommitmentWitnessStoneCompatible},
    SerializeError,
};
use crate::Felt252;

#[derive(Debug)]
pub struct TableCommitmentConfigStoneCompatible {
    pub n_columns: Felt252,
    pub vector: VectorCommitmentConfigStoneCompatible,
}
impl TryFrom<&StarkProof<Stark252PrimeField, Stark252PrimeField>>
    for TableCommitmentConfigStoneCompatible
{
    type Error = SerializeError;

    fn try_from(
        value: &StarkProof<Stark252PrimeField, Stark252PrimeField>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub struct TableDecommitmentStoneCompatible {
    pub n_values: Felt252,
    pub values: Vec<Felt252>,
}
impl TryFrom<&StarkProof<Stark252PrimeField, Stark252PrimeField>>
    for TableDecommitmentStoneCompatible
{
    type Error = SerializeError;

    fn try_from(
        value: &StarkProof<Stark252PrimeField, Stark252PrimeField>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub struct TableCommitmentWitnessStoneCompatible {
    pub vector: VectorCommitmentWitnessStoneCompatible,
}
impl TryFrom<&StarkProof<Stark252PrimeField, Stark252PrimeField>>
    for TableCommitmentWitnessStoneCompatible
{
    type Error = SerializeError;

    fn try_from(
        value: &StarkProof<Stark252PrimeField, Stark252PrimeField>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}
