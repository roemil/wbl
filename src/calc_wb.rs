use crate::{planes::PlaneProperties, ViktArm};

pub trait WeightAndBalance {
    fn is_weight_and_balance_ok(&self, prop: &PlaneProperties) -> bool;
    fn is_landing_weight_and_balance_ok(&self, prop: &PlaneProperties) -> bool;
    fn calc_weight_and_balance(&self, prop: &PlaneProperties) -> ViktArm;
    fn calc_landing_weight_and_balance(&self, prop: &PlaneProperties) -> ViktArm;
}
