use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use stark_platinum_prover::proof::stark::StarkProof;

use crate::Felt252;

use super::{
    fri::{FriConfigStoneCompatible, FriUnsentCommitmentStoneCompatible, FriWitnessStoneCompatible},
    pow::{ProofOfWorkConfigStoneCompatible, ProofOfWorkUnsentCommitmentStoneCompatible},
    table::{
        TableCommitmentConfigStoneCompatible, TableCommitmentWitnessStoneCompatible,
        TableDecommitmentStoneCompatible,
    },
    traces::{
        TracesConfigStoneCompatible, TracesDecommitmentStoneCompatible,
        TracesUnsentCommitmentStoneCompatible, TracesWitnessStoneCompatible,
    },
};

#[derive(Debug)]
pub struct StarkProofStoneCompatible {
    pub config: StarkConfigStoneCompatible,
    pub public_input: PublicInputStoneCompatible,
    pub unsent_commitment: StarkUnsentCommitmentStoneCompatible,
    pub witness: StarkWitnessStoneCompatible,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for StarkProofStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct StarkConfigStoneCompatible {
    pub traces: TracesConfigStoneCompatible,
    pub composition: TableCommitmentConfigStoneCompatible,
    pub fri: FriConfigStoneCompatible,
    pub proof_of_work: ProofOfWorkConfigStoneCompatible,
    // Log2 of the trace domain size.
    pub log_trace_domain_size: Felt252,
    // Number of queries to the last component, FRI.
    pub n_queries: Felt252,
    // Log2 of the number of cosets composing the evaluation domain, where the coset size is the
    // trace length.
    pub log_n_cosets: Felt252,
    // Number of layers that use a verifier friendly hash in each commitment.
    pub n_verifier_friendly_commitment_layers: Felt252,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for StarkConfigStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct PublicInputStoneCompatible {
    pub log_n_steps: Felt252,
    pub range_check_min: Felt252,
    pub range_check_max: Felt252,
    pub layout: Felt252,
    pub dynamic_params: Vec<Felt252>,
    pub n_segments: Felt252,
    pub segments: Vec<Felt252>,
    pub padding_addr: Felt252,
    pub padding_value: Felt252,
    pub main_page_len: Felt252,
    pub main_page: Vec<Felt252>,
    pub n_continuous_pages: Felt252,
    pub continuous_page_headers: Vec<Felt252>,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for PublicInputStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct StarkUnsentCommitmentStoneCompatible {
    pub traces: TracesUnsentCommitmentStoneCompatible,
    pub composition: Felt252,
    pub oods_values: Vec<Felt252>,
    pub fri: FriUnsentCommitmentStoneCompatible,
    pub proof_of_work: ProofOfWorkUnsentCommitmentStoneCompatible,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for StarkUnsentCommitmentStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct StarkWitnessStoneCompatible {
    pub traces_decommitment: TracesDecommitmentStoneCompatible,
    pub traces_witness: TracesWitnessStoneCompatible,
    pub composition_decommitment: TableDecommitmentStoneCompatible,
    pub composition_witness: TableCommitmentWitnessStoneCompatible,
    pub fri_witness: FriWitnessStoneCompatible,
}
impl From<&StarkProof<Stark252PrimeField, Stark252PrimeField>> for StarkWitnessStoneCompatible {
    fn from(value: &StarkProof<Stark252PrimeField, Stark252PrimeField>) -> Self {
        todo!()
    }
}
