use reth_primitives::{Parity as PrimitiveParity, Signature as PrimitiveSignature, TxType, U256};
use reth_rpc_types::{Parity, Signature};

/// Creates a new rpc signature from a legacy [primitive
/// signature](reth_primitives::Signature), using the give chain id to compute the signature's
/// recovery id.
///
/// If the chain id is `Some`, the recovery id is computed according to [EIP-155](https://eips.ethereum.org/EIPS/eip-155).
pub(crate) fn from_legacy_primitive_signature(
    signature: PrimitiveSignature,
    chain_id: Option<u64>,
) -> Signature {
    Signature {
        r: signature.r(),
        s: signature.s(),
        v: U256::from(legacy_parity(&signature, chain_id).to_u64()),
        y_parity: None,
    }
}

/// Creates a new rpc signature from a non-legacy [primitive
/// signature](reth_primitives::Signature). This sets the `v` value to `0` or `1` depending on
/// the signature's `odd_y_parity`.
pub(crate) fn from_typed_primitive_signature(signature: PrimitiveSignature) -> Signature {
    Signature {
        r: signature.r(),
        s: signature.s(),
        v: U256::from(signature.v().y_parity_byte()),
        y_parity: Some(Parity(signature.v().y_parity())),
    }
}

/// Creates a new rpc signature from a legacy [primitive
/// signature](reth_primitives::Signature).
///
/// The tx type is used to determine whether or not to use the `chain_id` to compute the
/// signature's recovery id.
///
/// If the transaction is a legacy transaction, it will use the `chain_id` to compute the
/// signature's recovery id. If the transaction is a typed transaction, it will set the `v`
/// value to `0` or `1` depending on the signature's `odd_y_parity`.
pub(crate) fn from_primitive_signature(
    signature: PrimitiveSignature,
    tx_type: TxType,
    chain_id: Option<u64>,
) -> Signature {
    match tx_type {
        TxType::Legacy => from_legacy_primitive_signature(signature, chain_id),
        _ => from_typed_primitive_signature(signature),
    }
}

/// Returns [PrimitiveParity] value based on `chain_id` for legacy transaction signature.
fn legacy_parity(signature: &PrimitiveSignature, chain_id: Option<u64>) -> PrimitiveParity {
    let odd_y_parity = signature.v().y_parity();
    if let Some(chain_id) = chain_id {
        PrimitiveParity::Parity(odd_y_parity).with_chain_id(chain_id)
    } else {
        #[cfg(feature = "optimism")]
        // pre bedrock system transactions were sent from the zero address as legacy
        // transactions with an empty signature
        //
        // NOTE: this is very hacky and only relevant for op-mainnet pre bedrock
        if *self == Self::optimism_deposit_tx_signature() {
            return Parity::Parity(false)
        }
        PrimitiveParity::NonEip155(odd_y_parity)
    }
}
