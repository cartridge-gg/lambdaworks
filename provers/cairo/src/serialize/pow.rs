use crate::Felt252;

#[derive(Debug)]
pub struct ProofOfWorkConfigStoneCompatible {
    // Proof of work difficulty (number of bits required to be 0).
    pub n_bits: Felt252,
}
// impl IntoProofOfWorkConfig of Into<ProofOfWorkConfigStoneCompatible, ProofOfWorkConfig> {
//     fn into(self: ProofOfWorkConfigStoneCompatible) -> ProofOfWorkConfig {
//         ProofOfWorkConfig { n_bits: self.n_bits.try_into().unwrap(), }
//     }
// }

#[derive(Debug)]
pub struct ProofOfWorkUnsentCommitmentStoneCompatible {
    pub nonce: Felt252,
}
// impl IntoProofOfWorkUnsentCommitment of Into<
//     ProofOfWorkUnsentCommitmentStoneCompatible, ProofOfWorkUnsentCommitment
// > {
//     fn into(self: ProofOfWorkUnsentCommitmentStoneCompatible) -> ProofOfWorkUnsentCommitment {
//         ProofOfWorkUnsentCommitment { nonce: self.nonce.try_into().unwrap(), }
//     }
// }
