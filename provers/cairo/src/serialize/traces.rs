use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use stark_platinum_prover::proof::stark::StarkProof;

use super::{
    table::{
        TableCommitmentConfigStoneCompatible, TableCommitmentWitnessStoneCompatible,
        TableDecommitmentStoneCompatible,
    },
    SerializeError,
};
use crate::Felt252;

#[derive(Debug)]
pub struct TracesConfigStoneCompatible {
    pub original: TableCommitmentConfigStoneCompatible,
    pub interaction: TableCommitmentConfigStoneCompatible,
}
impl TryFrom<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for TracesConfigStoneCompatible {
    type Error = SerializeError;

    fn try_from(
        value: &StarkProof<Stark252PrimeField, Stark252PrimeField>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub struct TracesDecommitmentStoneCompatible {
    pub original: TableDecommitmentStoneCompatible,
    pub interaction: TableDecommitmentStoneCompatible,
}
impl TryFrom<&StarkProof<Stark252PrimeField, Stark252PrimeField>>
    for TracesDecommitmentStoneCompatible
{
    type Error = SerializeError;

    fn try_from(
        value: &StarkProof<Stark252PrimeField, Stark252PrimeField>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub struct TracesUnsentCommitmentStoneCompatible {
    pub original: Felt252,
    pub interaction: Felt252,
}
impl TryFrom<&StarkProof<Stark252PrimeField, Stark252PrimeField>>
    for TracesUnsentCommitmentStoneCompatible
{
    type Error = SerializeError;

    fn try_from(
        value: &StarkProof<Stark252PrimeField, Stark252PrimeField>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub struct TracesWitnessStoneCompatible {
    pub original: TableCommitmentWitnessStoneCompatible,
    pub interaction: TableCommitmentWitnessStoneCompatible,
}
impl TryFrom<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for TracesWitnessStoneCompatible {
    type Error = SerializeError;

    fn try_from(
        value: &StarkProof<Stark252PrimeField, Stark252PrimeField>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}
