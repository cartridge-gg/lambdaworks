use crate::Felt252;
use super::table::{TableCommitmentConfigStoneCompatible, TableDecommitmentStoneCompatible, TableCommitmentWitnessStoneCompatible};

#[derive(Debug)]
pub struct TracesConfigStoneCompatible {
    pub original: TableCommitmentConfigStoneCompatible,
    pub interaction: TableCommitmentConfigStoneCompatible,
}
// impl IntoTracesConfig of Into<TracesConfigStoneCompatible, TracesConfig> {
//     fn into(self: TracesConfigStoneCompatible) -> TracesConfig {
//         TracesConfig { original: self.original.into(), interaction: self.interaction.into() }
//     }
// }

#[derive(Debug)]
pub struct TracesDecommitmentStoneCompatible {
    pub original: TableDecommitmentStoneCompatible,
    pub interaction: TableDecommitmentStoneCompatible,
}
// impl IntoTracesDecommitment of Into<TracesDecommitmentStoneCompatible, TracesDecommitment> {
//     fn into(self: TracesDecommitmentStoneCompatible) -> TracesDecommitment {
//         TracesDecommitment { original: self.original.into(), interaction: self.interaction.into() }
//     }
// }

#[derive(Debug)]
pub struct TracesUnsentCommitmentStoneCompatible {
    pub original: Felt252,
    pub interaction: Felt252,
}
// impl IntoTracesUnsentCommitment of Into<TracesUnsentCommitmentStoneCompatible, TracesUnsentCommitment> {
//     fn into(self: TracesUnsentCommitmentStoneCompatible) -> TracesUnsentCommitment {
//         TracesUnsentCommitment { original: self.original, interaction: self.interaction }
//     }
// }

#[derive(Debug)]
pub struct TracesWitnessStoneCompatible {
    pub original: TableCommitmentWitnessStoneCompatible,
    pub interaction: TableCommitmentWitnessStoneCompatible,
}
// impl IntoTracesWitness of Into<TracesWitnessStoneCompatible, TracesWitness> {
//     fn into(self: TracesWitnessStoneCompatible) -> TracesWitness {
//         TracesWitness { original: self.original.into(), interaction: self.interaction.into() }
//     }
// }
