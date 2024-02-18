use crate::Felt252;

#[derive(Debug)]
pub struct VectorCommitmentConfigStoneCompatible {
    pub height: Felt252,
    pub n_verifier_friendly_commitment_layers: Felt252,
}
// impl IntoVectorCommitmentConfig of Into<VectorCommitmentConfigStoneCompatible, VectorCommitmentConfig> {
//     fn into(self: VectorCommitmentConfigStoneCompatible) -> VectorCommitmentConfig {
//         VectorCommitmentConfig {
//             height: self.height,
//             n_verifier_friendly_commitment_layers: self.n_verifier_friendly_commitment_layers,
//         }
//     }
// }

#[derive(Debug)]
pub struct VectorCommitmentWitnessStoneCompatible {
    pub n_authentications: Felt252,
    pub authentications: Vec<Felt252>,
}
// impl IntoVectorCommitmentWitness of Into<
//     VectorCommitmentWitnessStoneCompatible, VectorCommitmentWitness
// > {
//     fn into(self: VectorCommitmentWitnessStoneCompatible) -> VectorCommitmentWitness {
//         VectorCommitmentWitness { authentications: self.authentications.span(), }
//     }
// }
