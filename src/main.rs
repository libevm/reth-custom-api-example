mod apis;

use apis::gasused::{CustomEthNamespaceExt, CustomEthNamespaceServer};
use clap::Parser;
use reth::cli::components::{RethNodeComponents, RethRpcComponents};
use reth::cli::{
    config::RethRpcConfig,
    ext::{RethCliExt, RethNodeCommandConfig},
    Cli,
};
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
    fn extend_rpc_modules<Conf, Reth>(
        &mut self,
        _config: &Conf,
        components: &Reth,
        rpc_components: RethRpcComponents<'_, Reth>,
    ) -> eyre::Result<()>
    where
        Conf: RethRpcConfig,
        Reth: RethNodeComponents,
    {
        if self.extend_eth_namespace {
            let provider = components.provider();
            let bundle_ext = CustomEthNamespaceExt::new(provider);
            rpc_components
                .modules
                .merge_configured(bundle_ext.into_rpc())?;
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
