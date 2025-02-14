use clap::ArgMatches;

pub fn execute(_matches: &ArgMatches) -> anyhow::Result<()> {
    // TODO: this is not proper error handling, fix it
    superr_inspect::launch().unwrap();

    Ok(())
}
