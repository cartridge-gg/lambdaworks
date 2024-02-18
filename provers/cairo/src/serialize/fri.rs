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
// impl IntoFriConfig of Into<FriConfigStoneCompatible, FriConfig> {
//     fn into(self: FriConfigStoneCompatible) -> FriConfig {
//         let mut inner_layers = VecTrait::<TableCommitmentConfig>::new();
//         let mut i = 0;
//         loop {
//             if i == self.inner_layers.len() {
//                 break;
//             }

//             inner_layers
//                 .append(
//                     TableCommitmentConfig {
//                         n_columns: *self.inner_layers.at(i),
//                         vector: VectorCommitmentConfig {
//                             height: *self.inner_layers.at(i + 1),
//                             n_verifier_friendly_commitment_layers: *self.inner_layers.at(i + 2),
//                         }
//                     }
//                 );
//             i += 3;
//         };
//         FriConfig {
//             log_input_size: self.log_input_size,
//             n_layers: self.n_layers,
//             inner_layers: inner_layers.span(),
//             fri_step_sizes: self.fri_step_sizes.span(),
//             log_last_layer_degree_bound: self.log_last_layer_degree_bound,
//         }
//     }
// }

#[derive(Debug)]
pub struct FriUnsentCommitmentStoneCompatible {
    pub inner_layers: Vec<Felt252>,
    pub last_layer_coefficients: Vec<Felt252>,
}
// impl IntoFriUnsentCommitment of Into<FriUnsentCommitmentStoneCompatible, FriUnsentCommitment> {
//     fn into(self: FriUnsentCommitmentStoneCompatible) -> FriUnsentCommitment {
//         let mut inner_layers = VecTrait::<Felt252>::new();
//         let mut i = 0;
//         loop {
//             if i == self.inner_layers.len() {
//                 break;
//             }
//             inner_layers.append(*self.inner_layers[i]);
//             i += 1;
//         };
//         FriUnsentCommitment {
//             inner_layers: inner_layers.span(),
//             last_layer_coefficients: self.last_layer_coefficients.span(),
//         }
//     }
// }

#[derive(Debug)]
pub struct FriWitnessStoneCompatible {
    pub layers: Vec<Felt252>,
}
// impl IntoFriWitness of Into<FriWitnessStoneCompatible, FriWitness> {
//     fn into(self: FriWitnessStoneCompatible) -> FriWitness {
//         let layers_span = self.layers.span();
//         let mut layers = VecTrait::<FriLayerWitness>::new();
//         let mut i = 0;
//         loop {
//             if i == layers_span.len() {
//                 break;
//             }

//             let n = *layers_span[i];
//             i += 1;
//             let mut leaves = VecTrait::<Felt252>::new();
//             let mut j = 0;
//             loop {
//                 if j == n {
//                     break;
//                 }

//                 leaves.append(*layers_span[i]);
//                 i += 1;
//                 j += 1;
//             };

//             let n = *layers_span[i];
//             i += 1;
//             let mut authentications = VecTrait::<Felt252>::new();
//             let mut j = 0;
//             loop {
//                 if j == n {
//                     break;
//                 }
//                 authentications.append(*layers_span[i]);
//                 i += 1;
//                 j += 1;
//             };

//             layers
//                 .append(
//                     FriLayerWitness {
//                         leaves: leaves.span(),
//                         table_witness: TableCommitmentWitness {
//                             vector: VectorCommitmentWitness {
//                                 authentications: authentications.span(),
//                             }
//                         },
//                     }
//                 );
//         };

//         FriWitness { layers: layers.span(), }
//     }
// }
