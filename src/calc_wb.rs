use crate::{planes::PlaneProperties, FailReason, WeightLever};

pub trait WeightAndBalance {
    fn is_weight_and_balance_ok(&self, prop: &PlaneProperties) ->  Result<(), FailReason>;
    fn is_landing_weight_and_balance_ok(&self, prop: &PlaneProperties) -> Result<(), FailReason>;
    fn calc_weight_and_balance(&self, prop: &PlaneProperties) -> WeightLever;
    fn calc_landing_weight_and_balance(&self, prop: &PlaneProperties) -> WeightLever;
}
