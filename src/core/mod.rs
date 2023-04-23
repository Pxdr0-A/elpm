use std::collections::HashMap;

mod random;

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
        match TwoDimVec::check_elm_search(self, i, j) {
            Ok(()) => {},
            Err(SearchError::IndexError(value)) => {
                panic!("Index out of bonds for axis {}.", value)
            }
        }

        let elm = self.body[i*self.shape[1] + j];

        elm
    }

    fn check_elm_search(data: &TwoDimVec, i: &usize, j: &usize) -> Result<(), SearchError> {
        if data.shape[0] < *i {
            Err(SearchError::IndexError(0))
        } else if data.shape[1] < *j {
            Err(SearchError::IndexError(1))
        } else {
            Ok(())
        }
    }

    pub fn row(&self, i: &usize) -> Vec<f64> {
        //check validity of the search
        match TwoDimVec::check_row_search(self, i) {
            Ok(()) => {},
            Err(SearchError::IndexError(value)) => {
                panic!("Index out of bonds for axis {}.", value)
            }
        }

        let init = i*self.shape[1];
        let end = i*self.shape[1] + self.shape[1];

        let selection = Vec::from(&self.body[init..end]);

        selection
    }

    fn check_row_search(data: &TwoDimVec, i: &usize) -> Result<(), SearchError> {
        if data.shape[0] < *i {
            Err(SearchError::IndexError(0))
        } else {
            Ok(())
        }
    }

    pub fn add_row(&mut self, row: &mut Vec<f64>) {
        // call inside an expression where mut Vec<f64> is declared
        // add verification for capacity and insertion
        match TwoDimVec::check_addition(self, row.len()) {
            Ok(()) => {},
            Err(AdditionError::CapacityError(value)) => {
                panic!("Exceeded two dimensional vector capacity on axis {}.", value)
            },
            Err(AdditionError::IncoherentShapeError) => {
                panic!("Attempting to add a row with incompatible number of elements.")
            }
        }

        self.shape[0] += 1;
        self.shape[1] = row.len();
        self.body.append(row);
    }

    fn check_addition(data: &mut TwoDimVec, row_len: usize) -> Result<(), AdditionError> {
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

    pub fn sample(shape: [usize; 2], n_classes: usize) -> (HashMap<String, Vec<f64>>, NumericDataset) {
        let mut dataset = NumericDataset::new(shape);

        // build cluster centers
        let mut centers: HashMap<String, Vec<f64>> = HashMap::new();
        { // loop scope for contructing centers
            // i n classes
            for i in 0..n_classes {
                let mut added_val;
                let mut added_vec: Vec<f64> = Vec::with_capacity(shape[1]);
                // j coordenates (features)
                for j in 0..shape[1] {
                    // build center out of random numbers
                    (_, added_val) = random::lcg(
                        101_305u128 +
                        (i as u128) * 1_434u128 * (shape[1] as u128) +
                        (j as u128) * 157u128
                    );
                    added_vec.push(added_val * 100.0);
                }
                centers.insert(
                    format!("center {}", i).to_string(),
                    added_vec);
            }
        }

        let mut lcg_val: f64 = 0.0;
        let mut inner_count: f64 = 0.0;
        { // initial values (guarantee at least one point per class)
            let mut added_row: Vec<f64>;
            let mut count: f64 = 0.0;
            for center in centers.values() {
                added_row = center.into_iter().map(|x| {
                    (_, lcg_val) = random::lcg(
                        1_837u128 +
                        1_713 * (count as u128) +
                        192 * (inner_count as u128)
                    );
                    inner_count += 1.0;
                    x + if lcg_val > 0.5 {lcg_val} else {-lcg_val} * 10.0
                }).collect();
                dataset.add_row(
                    &mut added_row,
                    &count
                );
                
                count += 1.0;
            }

        }

        { // rest of the rows (n_classes were already done)
            let mut class_val: f64;
            let mut key: String;
            let mut center: &Vec<f64>;
            let mut added_row: Vec<f64>;
            let mut index: usize;
            for point in n_classes..shape[0] {
                (_, lcg_val) = random::lcg(
                    1_153u128 +
                    100 * (point as u128)
                );
                println!("{}", lcg_val);
                class_val = (lcg_val * (n_classes as f64 - 1.0)).round();
                key = format!("center {}", class_val as usize).to_string();

                // unwrap does not panic
                // except panics if the result is None
                center = centers.get(&key).expect("Did not find the value for the search key");
                index = 0;
                added_row = vec![1.0; center.len()].into_iter().map(|x| {
                    // go through the coordinates
                    (_, lcg_val) = random::lcg(
                        1_837u128 +
                        1_713 * (point as u128) +
                        192 * (inner_count as u128)
                    );
                    inner_count += 1.0;
                    index += 1;

                    x*center[index - 1] + if lcg_val > 0.5 {lcg_val} else {-lcg_val} * 10.0
                }).collect();

                dataset.add_row(&mut added_row, &class_val);
            }
        }

        (centers, dataset)
    }

    pub fn row(&self, i: &usize) -> (Vec<f64>, f64) {
        // check validity of the search
        match NumericDataset::check_search(self, i) {
            Ok(()) => {},
            Err(SearchError::IndexError(value)) => {
                panic!("Index out of bonds for axis {}.", value)
            }
        }

        let target_search = self.target[*i];
        let line_search = self.body.row(i);

        (line_search, target_search)
    }

    fn check_search(dataset: &NumericDataset, i: &usize) -> Result<(), SearchError> {
        if dataset.shape[0] < *i {
            Err(SearchError::IndexError(0))
        } else {
            Ok(())
        }
    }

    pub fn add_row(&mut self, row: &mut Vec<f64>, target_val: &f64) {
        // call inside an expression where mut Vec<f64> is declared
        // verification for capacity and insertion
        match NumericDataset::check_addition(self, row.len()) {
            Ok(()) => {},
            Err(AdditionError::CapacityError(value)) => {
                panic!("Exceeded dataset capacity on axis {}.", value)
            },
            Err(AdditionError::IncoherentShapeError) => {
                panic!("Attempting to add a row with incompatible number of elements.")
            }
        }

        self.shape[0] += 1;
        self.shape[1] = row.len();
        self.body.add_row(row);
        self.target.push(*target_val);
    }

    fn check_addition(dataset: &mut NumericDataset, row_len: usize) -> Result<(), AdditionError> {
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

enum SearchError {
    IndexError(usize)
}

enum AdditionError {
    CapacityError(usize),
    IncoherentShapeError
}