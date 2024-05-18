use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;

use rcli::{CmdExecutor, Opts};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let timer = time::format_description::parse(
        "[year]-[month padding:zero]-[day padding:zero]T[hour]:[minute]:[second].[subsecond digits:6][offset_hour sign:mandatory]:[offset_minute]",
    )?;
    let time_offset = time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(time_offset, timer);
    tracing_subscriber::fmt().with_timer(timer).init();

    let opts = Opts::parse();
    opts.cmd.execute().await?;
    Ok(())
}
