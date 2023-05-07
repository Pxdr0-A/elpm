use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use random_lcg::prelude::*;

#[derive(Debug)]
pub struct TwoDimVec {
    body: Vec<f64>,
    shape: [usize; 2],
    capacity: [usize; 2]
}

impl TwoDimVec {
    pub fn new(capacity: [usize; 2]) -> TwoDimVec {
        // allocates enough memory
        let body = Vec::with_capacity(capacity[0]*capacity[1]);
        let shape = [0, 0];
        TwoDimVec { body, shape, capacity }
    }

    pub fn elm(&self, i: &usize, j: &usize) -> f64 {
        // i - lines; j - columns
        // check for search validity
        match error_handling::check_elm_search(self, i, j) {
            Ok(()) => {},
            Err(error_handling::SearchError::IndexError(value)) => {
                panic!("Index out of bonds for axis {}.", value)
            }
        }

        let elm = self.body[i*self.shape[1] + j];

        elm
    }

    pub fn row(&self, i: &usize) -> Vec<f64> {
        //check validity of the search
        match error_handling::check_row_search(self, i) {
            Ok(()) => {},
            Err(error_handling::SearchError::IndexError(value)) => {
                panic!("Index out of bonds for axis {}.", value)
            }
        }

        let init = i*self.shape[1];
        let end = i*self.shape[1] + self.shape[1];

        let selection = Vec::from(&self.body[init..end]);

        selection
    }

    pub fn add_row(&mut self, row: &mut Vec<f64>) {
        // call inside an expression where mut Vec<f64> is declared
        // add verification for capacity and insertion
        match error_handling::check_addition(self, row.len()) {
            Ok(()) => {},
            Err(error_handling::AdditionError::CapacityError(value)) => {
                panic!("Exceeded two dimensional vector capacity on axis {}.", value)
            },
            Err(error_handling::AdditionError::IncoherentShapeError) => {
                panic!("Attempting to add a row with incompatible number of elements.")
            }
        }

        self.shape[0] += 1;
        self.shape[1] = row.len();
        self.body.append(row);
    }
}

#[derive(Debug)]
pub struct NumericDataset {
    body: TwoDimVec,
    target:Vec<f64>,
    shape: [usize; 2],
    capacity: [usize; 2]
}

impl NumericDataset {
    pub fn new(capacity: [usize; 2]) -> NumericDataset {
        let target = Vec::with_capacity(capacity[0]);
        let body = TwoDimVec::new(capacity);
        let shape = [0, 0];
        NumericDataset { body, target, shape, capacity }
    }

    pub fn sample(shape: [usize; 2], n_classes: usize, seed: u128) -> NumericDataset {
        let mut dataset = NumericDataset::new(shape);
        // build cluster centers
        let mut centers: HashMap<String, Vec<f64>> = HashMap::new();

        crate::core::build_random_centers(&mut centers, &shape, n_classes, seed);
        crate::core::add_random_points(&mut dataset, &mut centers, &shape, n_classes, seed);

        dataset
    }

    pub fn row(&self, i: &usize) -> (Vec<f64>, f64) {
        // check validity of the search
        match error_handling::check_search(self, i) {
            Ok(()) => {},
            Err(error_handling::SearchError::IndexError(value)) => {
                panic!("Index out of bonds for axis {}.", value)
            }
        }

        let target_search = self.target[*i];
        let line_search = self.body.row(i);

        (line_search, target_search)
    }

    pub fn add_row(&mut self, row: &mut Vec<f64>, target_val: &f64) {
        // call inside an expression where mut Vec<f64> is declared
        // verification for capacity and insertion
        match error_handling::check_dataset_addition(self, row.len()) {
            Ok(()) => {},
            Err(error_handling::AdditionError::CapacityError(value)) => {
                panic!("Exceeded dataset capacity on axis {}.", value)
            },
            Err(error_handling::AdditionError::IncoherentShapeError) => {
                panic!("Attempting to add a row with incompatible number of elements.")
            }
        }

        self.shape[0] += 1;
        self.shape[1] = row.len();
        self.body.add_row(row);
        self.target.push(*target_val);
    }
}

mod error_handling {
    pub enum SearchError {
        IndexError(usize)
    }

    pub enum AdditionError {
        CapacityError(usize),
    IncoherentShapeError
    }

    // TwoDimVec
    pub fn check_elm_search(data: &super::TwoDimVec, i: &usize, j: &usize) -> Result<(), SearchError> {
        if data.shape[0] < *i {
            Err(SearchError::IndexError(0))
        } else if data.shape[1] < *j {
            Err(SearchError::IndexError(1))
        } else {
            Ok(())
        }
    }

    pub fn check_row_search(data: &super::TwoDimVec, i: &usize) -> Result<(), SearchError> {
        if data.shape[0] < *i {
            Err(SearchError::IndexError(0))
        } else {
            Ok(())
        }
    }

    pub fn check_addition(data: &mut super::TwoDimVec, row_len: usize) -> Result<(), AdditionError> {
        if data.capacity[0] == data.shape[0] {
            Err(AdditionError::CapacityError(0))
        } else if data.capacity[1] < row_len && data.shape[1] == 0 {
            Err(AdditionError::CapacityError(1))
        } else if data.shape[1] != row_len && data.shape[1] != 0 {
            Err(AdditionError::IncoherentShapeError)
        } else {
            Ok(())
        }
    }

    // NumericDataset
    pub fn check_search(dataset: &super::NumericDataset, i: &usize) -> Result<(), SearchError> {
        if dataset.shape[0] < *i {
            Err(SearchError::IndexError(0))
        } else {
            Ok(())
        }
    }

    pub fn check_dataset_addition(dataset: &mut super::NumericDataset, row_len: usize) -> Result<(), AdditionError> {
        if dataset.capacity[0] == dataset.shape[0] {
            Err(AdditionError::CapacityError(0))
        } else if dataset.capacity[1] < row_len && dataset.shape[1] == 0 {
            Err(AdditionError::CapacityError(1))
        } else if dataset.shape[1] != row_len && dataset.shape[1] != 0 {
            Err(AdditionError::IncoherentShapeError)
        } else {
            Ok(())
        }
    }
}

fn build_random_centers(centers: &mut HashMap<String, Vec<f64>>,
                        shape: &[usize],
                        n_classes: usize,
                        mut next_val: u128) {
    // loop scope for contructing centers
    // i n classes
    let mut added_val: f64;
    for i in 0..n_classes {
        let mut added_vec: Vec<f64> = Vec::with_capacity(shape[1]);
        // j coordenates (features)
        for _ in 0..shape[1] {
            // build center out of random numbers
            (next_val, added_val) = lcg(next_val);
            added_vec.push(added_val * 100.0);
        }
        centers.insert(
            format!("center {}", i).to_string(),
                    added_vec);
    }
}

fn add_random_points(dataset: &mut NumericDataset,
                     centers: &mut HashMap<String, Vec<f64>>,
                     shape: &[usize],
                     n_classes: usize,
                     mut next_val: u128) {
    // rest of the rows (n_classes were already done)
    let mut class_val: f64;
    let mut lcg_val: f64;
    let mut key: String;
    let mut center: &Vec<f64>;
    let mut added_row: Vec<f64>;
    let mut index: usize;
    // for an additional random number
    let mut nanos: u128 = 0;
    for c in 0..shape[0] {
        (next_val, lcg_val) = lcg(next_val);

        // garantee the initial values are one of each class
        if c < n_classes {
            class_val = c as f64;
        } else {
            class_val = (lcg_val * (n_classes as f64 - 1.0)).round();
        }
        key = format!("center {}", class_val as usize).to_string();

        // unwrap does not panic
        // expect panics if the result is None
        center = centers.get(&key).expect("Did not find the value for the search key");
        index = 0;
        added_row = vec![1.0; center.len()].into_iter().map(|x| {
            // go through the coordinates
            (next_val, lcg_val) = lcg(next_val);
            index += 1;
            nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
            x*center[index - 1] + if nanos % 2 == 0 {lcg_val} else {-lcg_val} * ((nanos % 20) as f64)
        }).collect();

        dataset.add_row(&mut added_row, &class_val);
    }
}