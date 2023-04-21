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