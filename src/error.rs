use burn::tensor::DataError;
use mass_spectrometry::structs::similarity_errors::SimilarityComputationError;

use mascot_rs::error::MascotError;

#[derive(Debug)]
pub enum SpectraError {
    InvalidArray,
    Mascot(MascotError),
    SimilarityComputation(SimilarityComputationError),
    Io(std::io::Error),
    BurnData(DataError),
    Csv(csv::Error),
}

impl From<MascotError> for SpectraError {
    fn from(error: MascotError) -> Self {
        Self::Mascot(error)
    }
}

impl From<SimilarityComputationError> for SpectraError {
    fn from(error: SimilarityComputationError) -> Self {
        Self::SimilarityComputation(error)
    }
}

impl From<std::io::Error> for SpectraError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<DataError> for SpectraError {
    fn from(value: DataError) -> Self {
        Self::BurnData(value)
    }
}

impl From<csv::Error> for SpectraError {
    fn from(value: csv::Error) -> Self {
        Self::Csv(value)
    }
}

impl core::fmt::Display for SpectraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidArray => write!(f, "The array is invalid"),
            Self::Mascot(mascot_error) => write!(f, "MASCOT ERROR: {mascot_error}"),
            Self::SimilarityComputation(similarity_computation_error) => write!(
                f,
                "Similarity Computation Error: {similarity_computation_error}"
            ),
            Self::Io(error) => write!(f, "IO ERROR: {error}"),
            Self::BurnData(data_error) => write!(f, "BURN DATA ERROR: {data_error}"),
            Self::Csv(error) => write!(f, "CSV ERROR: {error}"),
        }
    }
}
