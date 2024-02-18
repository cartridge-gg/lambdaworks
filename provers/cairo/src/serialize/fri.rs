use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use stark_platinum_prover::proof::stark::StarkProof;

use crate::Felt252;

#[derive(Debug)]
pub struct FriConfigStoneCompatible {
    // Log2 of the size of the input layer to FRI.
    pub log_input_size: Felt252,
    // Number of layers in the FRI. Inner + last layer.
    pub n_layers: Felt252,
    // Vec of size n_layers - 1, each entry is a configuration of a table commitment for the
    // corresponding inner layer.
    pub inner_layers: Vec<Felt252>,
    // Vec of size n_layers, each entry represents the FRI step size,
    // i.e. the number of FRI-foldings between layer i and i+1.
    pub fri_step_sizes: Vec<Felt252>,
    pub log_last_layer_degree_bound: Felt252,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for FriConfigStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct FriUnsentCommitmentStoneCompatible {
    pub inner_layers: Vec<Felt252>,
    pub last_layer_coefficients: Vec<Felt252>,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>>
    for FriUnsentCommitmentStoneCompatible
{
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct FriWitnessStoneCompatible {
    pub layers: Vec<Felt252>,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for FriWitnessStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}
