#[cfg(feature = "sys")]
use crate::barretenberg_rs::composer::StandardComposer;
use acvm::acir::circuit::Circuit;

use acvm::SmartContract;

use super::Plonk;

impl SmartContract for Plonk {
    #[cfg(feature = "sys")]
    fn eth_contract_from_cs(&self, circuit: Circuit) -> String {
        let constraint_system = crate::serialise_circuit(&circuit);

        let mut composer = StandardComposer::new(constraint_system);

        composer.smart_contract()
    }

    #[cfg(feature = "wasm-base")]
    fn eth_contract_from_cs(&self, circuit: Circuit) -> String {
        todo!();
    }
}
