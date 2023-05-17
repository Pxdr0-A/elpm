use std::thread;
use std::thread::JoinHandle;

use super::NumericDataset;
use super::TwoDimVec;

#[derive(Debug)]
pub struct Ellipsoid {
    center: Vec<f64>,
    semi_axis: Vec<f64>,
    rotation: Vec<f64>
}

#[derive(Debug)]
pub struct EllipsoidModel {
    tol: f64,
    n_classes: usize,
    ellipses: Vec<Ellipsoid>
}

impl Ellipsoid {
    fn new(n_dim: usize) -> Ellipsoid {
        Ellipsoid {
            center: Vec::with_capacity(n_dim),
            semi_axis: Vec::with_capacity(n_dim),
            rotation: Vec::with_capacity(n_dim - 1)
        }
    }
}

impl EllipsoidModel {
    pub fn new(tol: f64, n_classes: usize, n_dim: usize) -> EllipsoidModel {
        let mut ellipses: Vec<Ellipsoid> = Vec::with_capacity(n_classes);
        for _ in 0..n_classes {
            ellipses.push(Ellipsoid::new(n_dim));
        }

        EllipsoidModel { tol, n_classes, ellipses }
    }

    pub fn fit(&mut self, dataset: NumericDataset) {
        // each element is a tuple respective to each class/cluster
        // each tuple contains
        // -- cluster body
        // -- cluster center
        // -- (minima_id, minima, maxima_id, maxima) per axis
        let metadata = arrange_dataset(self.n_classes, dataset);

        println!("{:?}", metadata);
    }

    /*
    pub fn predict() {
        
    }
    
    pub fn evaluate() {
        
    }
    */
}


fn arrange_dataset(n_classes: usize, dataset: NumericDataset) -> Vec<(TwoDimVec, Vec<f64>, Vec<(usize, f64, usize, f64)>)> {
    // gather all classes (TwoDimVec) with minima and maxima (tuple)
    let mut thread_classes: Vec<JoinHandle<(TwoDimVec, Vec<f64>, Vec<(usize, f64, usize, f64)>)>> = Vec::with_capacity(
        n_classes
    );

    let mut current_dataset: NumericDataset;
    for i in 0..n_classes {
        current_dataset = dataset.clone();
        // each thread will search for each class
        thread_classes.push(thread::spawn(
            move || -> (TwoDimVec, Vec<f64>, Vec<(usize, f64, usize, f64)>) {
                let class_values = current_dataset.select_class(&(i as f64));

                let mut thread_min_max: Vec<JoinHandle<(usize, f64, usize, f64)>> = Vec::with_capacity(
                    dataset.body.shape[1]
                );
                for j in 0..dataset.body.shape[1] {
                    let current_class_values = class_values.clone();
                    thread_min_max.push(thread::spawn(
                        move || -> (usize, f64, usize, f64) {
                            current_class_values.min_max_axis(&j)
                        }
                    ));
                }

                let gathered_min_max: Vec<(usize, f64, usize, f64)> = thread_min_max.into_iter().map(|x| {
                    x.join().unwrap()
                }).collect();

                let center = find_center(&class_values);

                (class_values, center, gathered_min_max)
            }
        ));
    }

    // gathering the search results from the threads
    let gathered_classes: Vec<(TwoDimVec, Vec<f64>, Vec<(usize, f64, usize, f64)>)> = thread_classes.into_iter().map(|x| {
        x.join().unwrap()
    }).collect();

    gathered_classes
}


fn find_center(cluster: &TwoDimVec) -> Vec<f64> {
    let mut center: Vec<f64> = vec![0.0; cluster.shape[1]];
    let mut current_row;
    let mut count: usize = 0;
    for i in 0..cluster.shape[0] {
        // get row i
        current_row = cluster.row(&i);
        // go through the coordinates
        center = center.into_iter().map(|x| {
            count += 1;

            x + current_row[count-1]
        }).collect();

        count = 0;
    }

    // go again through the coordinates to divide
    center = center.into_iter().map(|x| {
        x/(cluster.shape[0] as f64)
    }).collect();

    center
}

mod error_handling {
    enum ModelError {
        IncoherentDimension(usize, usize),
        IncompleteModel
    }
}