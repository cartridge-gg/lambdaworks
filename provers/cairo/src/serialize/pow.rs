use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use stark_platinum_prover::proof::stark::StarkProof;

use crate::Felt252;

use super::SerializeError;

#[derive(Debug)]
pub struct ProofOfWorkConfigStoneCompatible {
    // Proof of work difficulty (number of bits required to be 0).
    pub n_bits: Felt252,
}
impl TryFrom<&StarkProof<Stark252PrimeField, Stark252PrimeField>>
    for ProofOfWorkConfigStoneCompatible
{
    type Error = SerializeError;

    fn try_from(
        value: &StarkProof<Stark252PrimeField, Stark252PrimeField>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub struct ProofOfWorkUnsentCommitmentStoneCompatible {
    pub nonce: Felt252,
}
impl TryFrom<&StarkProof<Stark252PrimeField, Stark252PrimeField>>
    for ProofOfWorkUnsentCommitmentStoneCompatible
{
    type Error = SerializeError;

    fn try_from(
        value: &StarkProof<Stark252PrimeField, Stark252PrimeField>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}
