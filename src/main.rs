use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Intercept and log traffic to a PCI device")]
struct Args {
    #[arg(short, long)]
    sbdf: String
}

fn main() -> anyhow::Result<()> {
    let cli = Args::parse();

    Ok(())
}
