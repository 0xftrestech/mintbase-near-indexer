use std::env;
use std::io;

use clap::Clap;

use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

/// NEAR Indexer Example
/// Watches for stream of blocks from the chain
#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "Near Inc. <hello@nearprotocol.com>")]
pub(crate) struct Opts {
  /// Sets a custom config dir. Defaults to ~/.near/
  #[clap(short, long)]
  pub home_dir: Option<std::path::PathBuf>,
  #[clap(subcommand)]
  pub subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub(crate) enum SubCommand {
  /// Run NEAR Indexer Example. Start observe the network
  Run,
  /// Initialize necessary configs
  Init(InitConfigArgs),
}

#[derive(Clap, Debug)]
pub(crate) struct InitConfigArgs {
  /// chain/network id (localnet, testnet, devnet, betanet)
  #[clap(short, long)]
  pub chain_id: Option<String>,
  /// Account ID for the validator key
  #[clap(long)]
  pub account_id: Option<String>,
  /// Specify private key generated from seed (TESTING ONLY)
  #[clap(long)]
  pub test_seed: Option<String>,
  /// Number of shards to initialize the chain with
  #[clap(short, long, default_value = "1")]
  pub num_shards: u64,
  /// Makes block production fast (TESTING ONLY)
  #[clap(short, long)]
  pub fast: bool,
  /// Genesis file to use when initialize testnet (including downloading)
  #[clap(short, long)]
  pub genesis: Option<String>,
  #[clap(short, long)]
  /// Download the verified NEAR genesis file automatically.
  pub download: bool,
  /// Specify a custom download URL for the genesis-file.
  #[clap(long)]
  pub download_genesis_url: Option<String>,
}

pub(crate) fn init_logging(verbose: bool) {
  let mut env_filter = EnvFilter::new("tokio_reactor=info,near=info,stats=info,telemetry=info");

  if verbose {
    env_filter = env_filter
      .add_directive("cranelift_codegen=warn".parse().unwrap())
      .add_directive("cranelift_codegen=warn".parse().unwrap())
      .add_directive("h2=warn".parse().unwrap())
      .add_directive("trust_dns_resolver=warn".parse().unwrap())
      .add_directive("trust_dns_proto=warn".parse().unwrap());

    env_filter = env_filter.add_directive(LevelFilter::DEBUG.into());
  } else {
    env_filter = env_filter.add_directive(LevelFilter::WARN.into());
  }

  if let Ok(rust_log) = env::var("RUST_LOG") {
    for directive in rust_log.split(',').filter_map(|s| match s.parse() {
      Ok(directive) => Some(directive),
      Err(err) => {
        eprintln!("Ignoring directive `{}`: {}", s, err);
        None
      }
    }) {
      env_filter = env_filter.add_directive(directive);
    }
  }
  tracing_subscriber::fmt::Subscriber::builder()
    .with_env_filter(env_filter)
    .with_writer(io::stderr)
    .init();
}
