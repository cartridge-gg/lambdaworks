use crate::Felt252;
use super::vector::{VectorCommitmentConfigStoneCompatible, VectorCommitmentWitnessStoneCompatible};

#[derive(Debug)]
pub struct TableCommitmentConfigStoneCompatible {
    pub n_columns: Felt252,
    pub vector: VectorCommitmentConfigStoneCompatible,
}
// impl IntoTableCommitmentConfig of Into<TableCommitmentConfigStoneCompatible, TableCommitmentConfig> {
//     fn into(self: TableCommitmentConfigStoneCompatible) -> TableCommitmentConfig {
//         TableCommitmentConfig { n_columns: self.n_columns, vector: self.vector.into(), }
//     }
// }

#[derive(Debug)]
pub struct TableDecommitmentStoneCompatible {
    pub n_values: Felt252,
    pub values: Vec<Felt252>,
}
// impl IntoTableDecommitment of Into<TableDecommitmentStoneCompatible, TableDecommitment> {
//     fn into(self: TableDecommitmentStoneCompatible) -> TableDecommitment {
//         TableDecommitment { values: self.values.span(), }
//     }
// }

#[derive(Debug)]
pub struct TableCommitmentWitnessStoneCompatible {
    pub vector: VectorCommitmentWitnessStoneCompatible,
}
// impl IntoTableCommitmentWitness of Into<TableCommitmentWitnessStoneCompatible, TableCommitmentWitness> {
//     fn into(self: TableCommitmentWitnessStoneCompatible) -> TableCommitmentWitness {
//         TableCommitmentWitness { vector: self.vector.into(), }
//     }
// }
