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
// impl IntoStarkProof of Into<StarkProofStoneCompatible, StarkProof> {
//     fn into(self: StarkProofStoneCompatible) -> StarkProof {
//         StarkProof {
//             config: self.config.into(),
//             public_input: self.public_input.into(),
//             unsent_commitment: self.unsent_commitment.into(),
//             witness: self.witness.into(),
//         }
//     }
// }

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
// impl IntoStarkConfig of Into<StarkConfigStoneCompatible, StarkConfig> {
//     fn into(self: StarkConfigStoneCompatible) -> StarkConfig {
//         StarkConfig {
//             traces: self.traces.into(),
//             composition: self.composition.into(),
//             fri: self.fri.into(),
//             proof_of_work: self.proof_of_work.into(),
//             log_trace_domain_size: self.log_trace_domain_size,
//             n_queries: self.n_queries,
//             log_n_cosets: self.log_n_cosets,
//             n_verifier_friendly_commitment_layers: self.n_verifier_friendly_commitment_layers,
//         }
//     }
// }

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
// impl IntoPublicInput of Into<PublicInputStoneCompatible, PublicInput> {
//     fn into(self: PublicInputStoneCompatible) -> PublicInput {
//         let mut segments = VecTrait::<SegmentInfo>::new();
//         let mut i = 0;
//         loop {
//             if i == self.segments.len() {
//                 break;
//             }

//             segments
//                 .append(
//                     SegmentInfo { begin_addr: *self.segments[i], stop_ptr: *self.segments[i + 1], }
//                 );
//             i += 2;
//         };

//         let mut page = VecTrait::<AddrValue>::new();
//         let mut i = 0;
//         loop {
//             if i == self.main_page.len() {
//                 break;
//             }

//             page.append(AddrValue { address: *self.main_page[i], value: *self.main_page[i + 1], });

//             i += 2;
//         };

//         let mut continuous_page_headers = VecTrait::<ContinuousPageHeader>::new();
//         let mut i = 0;
//         loop {
//             if i == self.continuous_page_headers.len() {
//                 break;
//             }

//             continuous_page_headers
//                 .append(
//                     ContinuousPageHeader {
//                         start_address: *self.continuous_page_headers[i],
//                         size: *self.continuous_page_headers[i + 1],
//                         hash: (*self.continuous_page_headers[i + 2]).into(),
//                         prod: *self.continuous_page_headers[i + 3],
//                     }
//                 );

//             i += 4;
//         };
//         PublicInput {
//             log_n_steps: self.log_n_steps,
//             rc_min: self.range_check_min,
//             rc_max: self.range_check_max,
//             layout: self.layout,
//             dynamic_params: self.dynamic_params,
//             segments: segments,
//             padding_addr: self.padding_addr,
//             padding_value: self.padding_value,
//             main_page: page.into(),
//             continuous_page_headers: continuous_page_headers,
//         }
//     }
// }

#[derive(Debug)]
pub struct StarkUnsentCommitmentStoneCompatible {
    pub traces: TracesUnsentCommitmentStoneCompatible,
    pub composition: Felt252,
    pub oods_values: Vec<Felt252>,
    pub fri: FriUnsentCommitmentStoneCompatible,
    pub proof_of_work: ProofOfWorkUnsentCommitmentStoneCompatible,
}
// impl IntoStarkUnsentCommitment of Into<StarkUnsentCommitmentStoneCompatible, StarkUnsentCommitment> {
//     fn into(self: StarkUnsentCommitmentStoneCompatible) -> StarkUnsentCommitment {
//         StarkUnsentCommitment {
//             traces: self.traces.into(),
//             composition: self.composition,
//             oods_values: self.oods_values.span(),
//             fri: self.fri.into(),
//             proof_of_work: self.proof_of_work.into(),
//         }
//     }
// }

#[derive(Debug)]
pub struct StarkWitnessStoneCompatible {
    pub traces_decommitment: TracesDecommitmentStoneCompatible,
    pub traces_witness: TracesWitnessStoneCompatible,
    pub composition_decommitment: TableDecommitmentStoneCompatible,
    pub composition_witness: TableCommitmentWitnessStoneCompatible,
    pub fri_witness: FriWitnessStoneCompatible,
}
// impl IntoStarkWitness of Into<StarkWitnessStoneCompatible, StarkWitness> {
//     fn into(self: StarkWitnessStoneCompatible) -> StarkWitness {
//         StarkWitness {
//             traces_decommitment: self.traces_decommitment.into(),
//             traces_witness: self.traces_witness.into(),
//             composition_decommitment: self.composition_decommitment.into(),
//             composition_witness: self.composition_witness.into(),
//             fri_witness: self.fri_witness.into(),
//         }
//     }
// }
