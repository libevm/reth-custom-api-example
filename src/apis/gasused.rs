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
            Ok(Some(b)) => Ok(b.gas_used),
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

#[cfg(test)]
mod tests {
    use crate::{CustomEthNamespaceExt, CustomEthNamespaceServer};
    use reth_primitives::Block;
    use reth_primitives::Header;
    use reth_provider::test_utils::MockEthProvider;

    #[tokio::test]
    async fn test_get_gas_used() {
        let mock_provider = MockEthProvider::default();

        // Insert it into the mock provider
        let block = Block {
            header: Header {
                number: 42,
                gas_used: 42069,
                ..Default::default()
            },
            ..Default::default()
        };
        let block_hash = block.hash_slow();
        mock_provider.add_block(block_hash, block);

        let ext = CustomEthNamespaceExt::new(mock_provider);
        let gas_used = ext.get_gas_used_by_block(42.into()).unwrap();
        let err_found = ext.get_gas_used_by_block(43.into());

        assert_eq!(gas_used, 42069);
        assert!(err_found.is_err());
    }
}
