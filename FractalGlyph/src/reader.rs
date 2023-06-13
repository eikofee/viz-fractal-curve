use std::path::Path;

use csv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SampleRecord {
    pub(crate) id: usize,
    pub(crate) groundtruth: usize,
    pub(crate) prediction: usize,
    pub(crate) order: usize,
}

#[derive(Debug, Deserialize)]
pub struct SampleRecordBuggyDataset {
    pub(crate) id: f64,
    pub(crate) groundtruth: usize,
    pub(crate) prediction: usize,
    pub(crate) order: usize,
}

impl From<SampleRecordBuggyDataset> for SampleRecord {
    fn from(d: SampleRecordBuggyDataset) -> SampleRecord {
        SampleRecord {
            id: d.id as usize,
            groundtruth: d.groundtruth,
            prediction: d.prediction,
            order: d.order,
        }
    }
}

// Id de l'objet, GT, Prediction, position dans la matrice
pub fn read_file<P: AsRef<Path>>(path: &P) -> Vec<SampleRecord> {
    csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .from_path(path.as_ref())
        .unwrap()
        .deserialize()
        .map(|result| {
            let record: SampleRecordBuggyDataset = result.unwrap();
            record
        })
        .map(|buggy| SampleRecord::from(buggy))
        .collect::<Vec<SampleRecord>>()
}
