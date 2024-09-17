use crate::{planes::PlaneProperties, ViktArm};

pub trait CalcWeightAndBalance {
    fn is_weight_and_balance_ok(&self) -> bool;
    fn calc_weight_and_balance(&self) -> ViktArm;
}

pub trait WeightAndBalance {
    fn is_weight_and_balance_ok(&self, prop: &PlaneProperties) -> bool;
    fn calc_weight_and_balance(&self, prop: &PlaneProperties) -> ViktArm;
}
