mod apis;

use apis::gasused::{CustomEthNamespaceExt, CustomEthNamespaceServer};
use clap::Parser;
use reth::{
    cli::{
        config::RethRpcConfig,
        ext::{RethCliExt, RethNodeCommandConfig},
        Cli,
    },
    network::{NetworkInfo, Peers},
    providers::{BlockReaderIdExt, CanonStateSubscriptions, ReceiptProvider},
    rpc::builder::{RethModuleRegistry, TransportRpcModules},
    tasks::TaskSpawner,
};
use reth_transaction_pool::TransactionPool;
use tracing::info;

// ---- CLI Args ----
struct RethNodeCliExtended;

impl RethCliExt for RethNodeCliExtended {
    type Node = RethExtended;
}

/// Our custom cli args extension that adds one flag to reth default CLI.
#[derive(Debug, Clone, Default, clap::Args)]
struct RethExtended {
    /// Enables builder mode
    #[clap(long, short)]
    pub extend_eth_namespace: bool,
}

impl RethNodeCommandConfig for RethExtended {
    // This is the entrypoint for the CLI to extend the RPC server with custom rpc namespaces.
    fn extend_rpc_modules<Conf, Provider, Pool, Network, Tasks, Events>(
        &mut self,
        _config: &Conf,
        registry: &mut RethModuleRegistry<Provider, Pool, Network, Tasks, Events>,
        modules: &mut TransportRpcModules,
    ) -> eyre::Result<()>
    where
        Conf: RethRpcConfig,
        Provider: BlockReaderIdExt + ReceiptProvider + Clone + Unpin + 'static,
        Pool: TransactionPool + Clone + 'static,
        Network: NetworkInfo + Peers + Clone + 'static,
        Tasks: TaskSpawner + Clone + 'static,
        Events: CanonStateSubscriptions + Clone + 'static,
    {
        if self.extend_eth_namespace {
            let provider = registry.provider().clone();
            let bundle_ext = CustomEthNamespaceExt::new(provider);
            modules.merge_configured(bundle_ext.into_rpc())?;
            info!("Custom ETH Namespace enabled!");
        }

        Ok(())
    }
}

// ---- Entrypoint ----

fn main() {
    // Parse args
    Cli::<RethNodeCliExtended>::parse().run().unwrap();
}
