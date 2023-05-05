use common::{
    clap::{self, Parser},
    eyre::Result,
    tokio,
};
use schema_gen::schemas::Schemas;

/// Generate things from the Stencila Schema
#[derive(Parser)]
struct Args {
    /// Generate reference docs
    #[arg(long, default_value_t = true)]
    docs: bool,

    /// Generate JSON-LD context
    #[arg(long, default_value_t = true)]
    json_ld: bool,

    /// Generate JSON Schema
    #[arg(long, default_value_t = true)]
    json_schema: bool,

    /// Generate Rust types
    #[arg(long, default_value_t = true)]
    rust: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut schemas = Schemas::read().await?;
    insta::assert_debug_snapshot!(schemas.schemas.get_index(0));
    // schemas.check()?;
    // schemas.extend()?;
    // schemas.expand()?;

    // if args.docs {
    //     schemas.docs().await?;
    // }

    // if args.json_ld {
    //     schemas.json_ld().await?;
    // }

    // if args.json_schema {
    //     schemas.json_schema().await?;
    // }

    // if args.rust {
    //     schemas.rust().await?;
    // }

    Ok(())
}
