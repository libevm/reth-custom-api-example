use jsonrpsee::{core::RpcResult, proc_macros::rpc, types::ErrorObjectOwned};
use reth::providers::{BlockReaderIdExt, ReceiptProvider};
use reth_primitives::BlockNumberOrTag;
use tracing::error;

#[rpc[server, namespace="eth"]]
pub trait CustomEthNamespace {
    #[method(name = "getGasUsedByBlock")]
    fn get_gas_used_by_block(&self, block_number: BlockNumberOrTag) -> RpcResult<u64>;
}

pub struct CustomEthNamespaceExt<P> {
    provider: P,
}

impl<P> CustomEthNamespaceExt<P>
where
    P: BlockReaderIdExt + ReceiptProvider + Clone + Unpin + 'static,
{
    pub fn new(provider: P) -> CustomEthNamespaceExt<P> {
        Self { provider }
    }
}

impl<P> CustomEthNamespaceServer for CustomEthNamespaceExt<P>
where
    P: BlockReaderIdExt + ReceiptProvider + Clone + Unpin + 'static,
{
    fn get_gas_used_by_block(&self, bn: BlockNumberOrTag) -> RpcResult<u64> {
        match self.provider.block_by_number_or_tag(bn) {
            Ok(Some(b)) => match self.provider.receipts_by_block(b.number.into()) {
                Ok(Some(txs)) => {
                    let mut gas = 0;
                    for tx in txs {
                        if tx.cumulative_gas_used > gas {
                            gas = tx.cumulative_gas_used;
                        }
                    }
                    Ok(gas)
                }
                _ => {
                    error!("failed to retrieve transactions for bn {bn:?}");
                    Err(ErrorObjectOwned::owned(
                        -1,
                        "Failed to retrieve transactions",
                        None::<()>,
                    ))
                }
            },
            _ => {
                error!("unable to retrieve block {bn:?}");
                Err(ErrorObjectOwned::owned(
                    -1,
                    "Invalid blockTag provided",
                    None::<()>,
                ))
            }
        }
    }
}
