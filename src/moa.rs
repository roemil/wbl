use crate::{
    calc_wb::CalcWeightAndBalance, is_inside_polygon, two_seater::{TwoSeater, TwoSeaterBuilder}, UseFuel, ViktArm
};

#[derive(Debug, Clone)]
pub struct Moa {
    two_seater: TwoSeater,
    bagage_back: ViktArm,
    bagage_front: ViktArm,
    bagage_wings: ViktArm,
}
pub struct MoaBuilder {
    two_seater_builder: TwoSeaterBuilder,
    bagage_back: f32,
    bagage_front: f32,
    bagage_wings: f32,
}

impl std::default::Default for MoaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MoaBuilder {
    pub fn new() -> MoaBuilder {
        let mut two_seater_builder = TwoSeaterBuilder::default();
        two_seater_builder.base_weight(ViktArm::new(453.5, 172.9));
        MoaBuilder {
            two_seater_builder,
            bagage_back: 0.0,
            bagage_front: 0.0,
            bagage_wings: 0.0,
        }
    }

    pub fn fuel(mut self, fuel: f32) -> MoaBuilder {
        self.two_seater_builder.fuel(ViktArm::new(fuel, 160.0));
        self
    }
    pub fn bagage_back(mut self, bagage_back: f32) -> MoaBuilder {
        self.bagage_back = bagage_back;
        self
    }
    pub fn bagage_front(mut self, bagage_front: f32) -> MoaBuilder {
        self.bagage_front = bagage_front;
        self
    }
    pub fn bagage_wings(mut self, bagage_wings: f32) -> MoaBuilder {
        self.bagage_wings = bagage_wings;
        self
    }
    pub fn pic(mut self, w_pic: f32) -> MoaBuilder {
        self.two_seater_builder.pic(ViktArm::new(w_pic, 208.5));
        self
    }
    pub fn pax(mut self, pax: f32) -> MoaBuilder {
        self.two_seater_builder.pax(ViktArm::new(pax, 208.5));
        self
    }

    pub fn build(self) -> Moa {
        Moa {
            two_seater: self.two_seater_builder.build(),
            bagage_back: ViktArm::new(self.bagage_back, 252.0),
            bagage_front: ViktArm::new(self.bagage_front, 252.0),
            bagage_wings: ViktArm::new(self.bagage_wings, 202.5),
        }
    }
}

impl Moa {
    fn calc_wb_use_fuel(&self, use_fuel: UseFuel) -> ViktArm {
        let total_w = self.two_seater.sum_weight(use_fuel.clone())
            + self.bagage_back.weight
            + self.bagage_front.weight
            + self.bagage_wings.weight;
        assert!(total_w > 0.0);
        let total_torque = self.two_seater.sum_torque(use_fuel)
            + self.bagage_back.torque()
            + self.bagage_front.torque()
            + self.bagage_wings.torque();

        ViktArm {
            weight: total_w,
            lever: total_torque / total_w,
        }
    }

    fn is_max_wing_load_ok(&self) -> bool {
        self.two_seater.base_weight.weight
            + self.two_seater.w_pic.weight
            + self.two_seater.w_pax.weight
            + self.bagage_back.weight
            <= 660.0
    }

    fn is_mtow_ok(&self) -> bool {
        self.calc_wb().weight <= 750.0
    }

    fn is_zero_fuel_ok(&self) -> bool {
        let zero_fuel_point = self.calc_wb_use_fuel(UseFuel::No);
        is_inside_polygon(zero_fuel_point, self.get_polygon(), false)
    }

    fn is_bagage_ok(&self) -> bool {
        self.bagage_back.weight <= 15.0 && self.bagage_front.weight <= 1.0
    }

    fn is_bagage_in_wings_ok(&self) -> bool {
        self.bagage_wings.weight <= 40.0
    }
}

impl CalcWeightAndBalance for Moa {
    fn calc_wb(&self) -> ViktArm {
        self.calc_wb_use_fuel(UseFuel::Yes)
    }
    fn get_polygon(&self) -> Vec<ViktArm> {
        vec![
            ViktArm::new(490.0, 171.2),
            ViktArm::new(600.0, 171.2),
            ViktArm::new(750.0, 179.2),
            ViktArm::new(750.0, 184.0),
            ViktArm::new(600.0, 184.0),
            ViktArm::new(490.0, 184.0),
        ]
    }
    fn is_weight_and_balance_ok(&self) -> bool {
        let calc = self.calc_wb();
        if !self.is_mtow_ok()
            || !self.is_max_wing_load_ok()
            || !self.is_bagage_in_wings_ok()
            || !self.is_bagage_ok()
        {
            return false;
        }

        let points = self.get_polygon();

        is_inside_polygon(calc, points, false) && self.is_zero_fuel_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bagage_ok_no_bagage() {
        let moa = MoaBuilder::new().build();
        assert!(moa.is_bagage_ok());
    }
    #[test]
    fn bagage_ok_back_bagage() {
        let moa = MoaBuilder::new().bagage_back(10.0).build();
        assert!(moa.is_bagage_ok());
    }
    #[test]
    fn bagage_ok_front_bagage() {
        let moa = MoaBuilder::new().bagage_front(0.5).build();
        assert!(moa.is_bagage_ok());
    }
    #[test]
    fn bagage_ok_both_bagage() {
        let moa = MoaBuilder::new()
            .bagage_back(10.0)
            .bagage_front(0.5)
            .build();
        assert!(moa.is_bagage_ok());
    }
    #[test]
    fn bagage_nok_back_bagage() {
        let moa = MoaBuilder::new().bagage_back(41.0).build();
        assert!(!moa.is_bagage_ok());
    }
    #[test]
    fn bagage_nok_front_bagage() {
        let moa = MoaBuilder::new().bagage_front(1.5).build();
        assert!(!moa.is_bagage_ok());
    }
    #[test]
    fn bagage_nok_both_bagage() {
        let moa = MoaBuilder::new()
            .bagage_back(41.0)
            .bagage_front(1.5)
            .build();
        assert!(!moa.is_bagage_ok());
    }
}
