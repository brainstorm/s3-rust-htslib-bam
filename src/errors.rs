use snafu::Snafu;
use rust_htslib::bam;

#[derive(Debug, Snafu)]
//#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("Error opening BAM file: {}", source))]
    BamOpen { source: bam::errors::Error },

    #[snafu(display("Error reading BAM file: {}", source))]
    BamReading { source: bam::errors::Error },

    #[snafu(display("Error reading BAM file"))]
    BamReadingUnknown,

    #[snafu(display("Too many target names on header: {}", source))]
    TargetNamesTooLong { source: std::num::TryFromIntError },

    #[snafu(display("Error saving region: {}", source))]
    StoreSave { source: Box<dyn std::error::Error> },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;