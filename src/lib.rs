use std::convert::TryInto;

use kiddo::{float::kdtree::KdTree, SquaredEuclidean};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[macro_export]
macro_rules! kd_tree_impl {
    ($($dim:expr),*) => {
        $(
        paste::paste! {
            #[wasm_bindgen]
            pub struct [< KdTreeWasm $dim >] {
                tree: KdTree<f64, u64, $dim, 32, u16>,
            }


            #[wasm_bindgen]
            impl [< KdTreeWasm $dim >] {
                #[wasm_bindgen(constructor)]
                pub fn new(set: &[f64]) -> Self {
                    let entries: Vec<[f64; $dim]> = set.chunks_exact($dim).flat_map(TryInto::try_into).collect();
                    Self { tree: (&entries).into() }
                }

                pub fn size(&self) -> u64 {
                    self.tree.size()
                }

                pub fn best_n_within(
                    &self,
                    point: &[f64],
                    distance: f64,
                    n: usize,
                ) -> Result<Vec<u64>, String> {
                    let point: &[f64; $dim] = point
                        .try_into()
                        .map_err(|_| String::from("dimension mismatch"))?;
                    Ok(self
                        .tree
                        .best_n_within::<SquaredEuclidean>(point, distance, n)
                        .into_iter()
                        .map(|entry| entry.item)
                        .collect())
                }
            }
        })*
    };
}

kd_tree_impl!(2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
