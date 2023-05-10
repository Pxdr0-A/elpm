use std::thread;
use std::thread::JoinHandle;

use super::NumericDataset;
use super::TwoDimVec;

#[derive(Debug)]
pub struct Elipsoide {
    center: Vec<f64>,
    semiaxis: Vec<f64>,
    rotation: Vec<f64>
}

#[derive(Debug)]
pub struct ElipsoideModel {
    tol: f64,
    n_classes: usize,
    elps: Vec<Elipsoide>
}

impl Elipsoide {
    fn new(n_dim: usize) -> Elipsoide {
        Elipsoide {
            center: Vec::with_capacity(n_dim),
            semiaxis: Vec::with_capacity(n_dim),
            rotation: Vec::with_capacity(n_dim - 1)
        }
    }
}

impl ElipsoideModel {
    pub fn new(tol: f64, n_classes: usize, n_dim: usize) -> ElipsoideModel {
        let mut elps: Vec<Elipsoide> = Vec::with_capacity(n_classes);
        for _ in 0..n_classes {
            elps.push(Elipsoide::new(n_dim));
        }

        ElipsoideModel { tol, n_classes, elps }
    }

    pub fn fit(&mut self, dataset: NumericDataset) {
        // gather all classes
        let mut gathered_classes: Vec<JoinHandle<TwoDimVec>> = Vec::with_capacity(
            self.n_classes
        );

        let mut current_dataset: NumericDataset;
        for i in 0..self.n_classes {
            current_dataset = dataset.clone();
            // each thread will search for each class
            gathered_classes.push(thread::spawn(move || -> TwoDimVec {
                let result = current_dataset.select_class(i as f64);

                result
            }));
        }

        // gathering the search results from the threads
        let gathered_results: Vec<TwoDimVec> = gathered_classes.into_iter().map(|x| {
            x.join().unwrap()
        }).collect();
    }
    
    pub fn predict() {
        
    }
    
    pub fn evaluate() {
        
    }
}

mod error_handling {
    enum ModelError {
        IncoherentDimension(usize, usize),
        IncompleteModel
    }
}