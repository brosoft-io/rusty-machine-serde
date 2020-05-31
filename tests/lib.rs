#[macro_use]
extern crate rulinalg_serde;
extern crate rusty_machine_serde as rm;
extern crate num as libnum;

pub mod learning {
    mod dbscan;
    mod lin_reg;
    mod k_means;
    mod gp;
    mod knn;
    mod pca;

    pub mod optim {
    	mod grad_desc;
    }
}

pub mod datasets;
