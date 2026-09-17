mod types;

use clap::Parser;
use pci_driver::backends::vfio::VfioPciDevice;
use std::path::PathBuf;

use types::Sbdf;

#[derive(Parser)]
#[command(version, about = "Intercept and log traffic to a PCI device")]
struct Args {
    /// The SBDF of the target PCI device.
    #[arg(short, long)]
    device: Sbdf,

    /// Path where we'll create the vfio-user socket.
    #[arg(short, long)]
    socket: Option<String>,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Args::parse();

    let dev_syspath = validate(cli.device)?;
    tracing::info!("Opening device {dev_syspath:?}");

    let _device = VfioPciDevice::open(dev_syspath)?;

    Ok(())
}

/// Make sure this device exists and is bound to vfio-pci.
fn validate(sbdf: Sbdf) -> anyhow::Result<PathBuf> {
    let dev_syspath = PathBuf::from(format!("/sys/bus/pci/devices/{sbdf}"));
    if !dev_syspath.exists() {
        anyhow::bail!("Device {sbdf} does not exist");
    }

    let driver_path = dev_syspath.join("driver");
    if !driver_path.exists() {
        anyhow::bail!("Device {sbdf} is not bound to any driver");
    }

    let driver_link = driver_path.read_link().expect("Driver symlink is valid");
    let driver = driver_link.file_name().expect("Bound driver has a name");
    if driver != "vfio-pci" {
        anyhow::bail!("Device {sbdf} is bound to {driver:?}. Expected \"vfio-pci\". Aborting.");
    }

    Ok(dev_syspath)
}
