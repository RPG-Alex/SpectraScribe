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
    Csv(csv::Error)
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