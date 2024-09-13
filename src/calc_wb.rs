use crate::ViktArm;

pub trait CalcWeightAndBalance {
    fn is_weight_and_balance_ok(&self) -> bool;
    fn get_polygon(&self) -> Vec<ViktArm>;
    fn calc_weight_and_balance(&self) -> ViktArm;
}
