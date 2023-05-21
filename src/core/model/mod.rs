use std::thread;
use std::thread::JoinHandle;
use std::f64::consts::PI;

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
        // -- Vec with (minima_id, minima, maxima_id, maxima) per axis
        let metadata = arrange_dataset(self.n_classes, dataset);

        let mut class_data: &(TwoDimVec, Vec<f64>, Vec<(Vec<f64>, Vec<f64>)>);
        for c in 0..self.n_classes {
            class_data = &metadata[c];
            find_semi_axis(&class_data.1, &class_data.2)
        }
    }
}


fn arrange_dataset(n_classes: usize, dataset: NumericDataset) -> Vec<(TwoDimVec, Vec<f64>, Vec<(Vec<f64>, Vec<f64>)>)> {
    // gather all classes (TwoDimVec) with minima and maxima (tuple)
    let mut thread_classes: Vec<JoinHandle<(TwoDimVec, Vec<f64>, Vec<(Vec<f64>, Vec<f64>)>)>> = Vec::with_capacity(
        n_classes
    );

    let mut current_dataset: NumericDataset;
    for i in 0..n_classes {
        current_dataset = dataset.clone();
        // each thread will search for each class
        thread_classes.push(thread::spawn(
            move || -> (TwoDimVec, Vec<f64>, Vec<(Vec<f64>, Vec<f64>)>) {
                let class_values = current_dataset.select_class(&(i as f64));

                let mut thread_min_max: Vec<JoinHandle<(Vec<f64>, Vec<f64>)>> = Vec::with_capacity(
                    dataset.body.shape[1]
                );
                for j in 0..dataset.body.shape[1] {
                    let current_class_values = class_values.clone();
                    thread_min_max.push(thread::spawn(
                        move || -> (Vec<f64>, Vec<f64>) {
                            current_class_values.min_max_axis(&j)
                        }
                    ));
                }

                let gathered_min_max: Vec<(Vec<f64>, Vec<f64>)> = thread_min_max.into_iter().map(|x| {
                    x.join().unwrap()
                }).collect();

                let center = find_center(&class_values);

                (class_values, center, gathered_min_max)
            }
        ));
    }

    // gathering the search results from the threads
    let gathered_classes: Vec<(TwoDimVec, Vec<f64>, Vec<(Vec<f64>, Vec<f64>)>)> = thread_classes.into_iter().map(|x| {
        x.join().unwrap()
    }).collect();

    gathered_classes
}

fn find_semi_axis(center: &Vec<f64>, limits: &Vec<(Vec<f64>, Vec<f64>)>) {
    let mut raw_semi_axis: Vec<Vec<f64>> = Vec::with_capacity(limits.len());
    for axis in 0..limits.len() {
        let min_diff = super::arithmetic::sub(
            center, &limits[axis].0);
        let min_length = super::arithmetic::norm(min_diff);

        let max_diff = super::arithmetic::sub(
            center, &limits[axis].1);
        let max_length = super::arithmetic::norm(max_diff);

        if min_length > max_length {
            raw_semi_axis.push(min_diff.clone());
        } else {
            raw_semi_axis.push(max_diff.clone());
        }
    }

    // adjusting semi axis
    let mut current_angle: f64;
    let delta: f64 = PI/100;
    for axis in 1..raw_semi_axis.len() {
        // manage from axis 0
        // check if current axis relative angle is lower or higher than pi/2
        // add or sub until tolerance for pi/2 is reached
        // consider dimension multiplicity in axis 0
        // this axis needs to be weighted less since it is going to change d-1 times
        current_angle = super::arithmetic::angle(
            raw_semi_axis[0].clone(),
            raw_semi_axis[axis].clone()
        );

        println!("{}", current_angle);

        if current_angle > PI/2 {

        }

    }
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