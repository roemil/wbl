use crate::{
    calc_wb::CalcWeightAndBalance,
    four_seater::{FourSeater, FourSeaterBuilder},
    is_inside_polygon,
    ViktArm,
};

#[derive(Debug, Clone)]
pub struct Ken {
    four_seater: FourSeater,
    bagage: ViktArm,
}

#[derive(Default)]
pub struct KenBuilder {
    four_seater_builder: FourSeaterBuilder,
    bagage: f32,
}

impl KenBuilder {
    pub fn new() -> KenBuilder {
        let mut four_seater_builder = FourSeaterBuilder::default();
        four_seater_builder.base_weight(ViktArm::new(685.2, 219.4));
        KenBuilder {
            four_seater_builder,
            bagage: 0.0,
        }
    }

    pub fn fuel(mut self, fuel: f32) -> KenBuilder {
        self.four_seater_builder.fuel(ViktArm::new(fuel, 241.3));
        self
    }
    pub fn bagage(mut self, bagage: f32) -> KenBuilder {
        self.bagage = bagage;
        self
    }
    pub fn pic(mut self, w_pic: f32) -> KenBuilder {
        self.four_seater_builder.pic(ViktArm::new(w_pic, 204.4));
        self
    }
    pub fn pax_front(mut self, pax: f32) -> KenBuilder {
        self.four_seater_builder.pax_front(ViktArm::new(pax, 204.4));
        self
    }
    pub fn pax_left_back(mut self, pax: f32) -> KenBuilder {
        self.four_seater_builder
            .pax_left_back(ViktArm::new(pax, 300.0));
        self
    }
    pub fn pax_right_back(mut self, pax: f32) -> KenBuilder {
        self.four_seater_builder
            .pax_right_back(ViktArm::new(pax, 300.0));
        self
    }

    pub fn build(self) -> Ken {
        Ken {
            four_seater: self.four_seater_builder.build(),
            bagage: ViktArm::new(self.bagage, 362.7),
        }
    }
}

impl Ken {
    fn is_mtow_ok(&self) -> bool {
        self.calc_wb().weight <= 1055.0
    }

    fn is_bagage_ok(&self) -> bool {
        self.bagage.weight <= 15.0
    }
    fn is_fuel_ok(&self) -> bool {
        self.four_seater.fuel.weight <= 129.0
    }
}

impl CalcWeightAndBalance for Ken {
    fn calc_wb(&self) -> ViktArm {
        let total_w = self.four_seater.sum_weight() + self.bagage.weight;
        assert!(total_w > 0.0);

        let total_torque = self.four_seater.sum_torque() + self.bagage.torque();

        ViktArm {
            weight: total_w,
            lever: total_torque / total_w,
        }
    }
    // TODO: Config can be read from json file
    fn get_polygon(&self) -> Vec<ViktArm> {
        vec![
            ViktArm::new(685.2, 210.8),
            ViktArm::new(885.0, 210.8),
            ViktArm::new(1055.0, 221.0),
            ViktArm::new(1055.0, 236.2),
            ViktArm::new(1055.0, 236.2),
            ViktArm::new(685.2, 236.2),
        ]
    }
    fn is_weight_and_balance_ok(&self) -> bool {
        let calc = self.calc_wb();
        if !self.is_mtow_ok() || !self.is_bagage_ok() || !self.is_fuel_ok() {
            return false;
        }

        let points = self.get_polygon();

        is_inside_polygon(calc, points, false)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bagage_ok_no_bagage() {
        let ken = KenBuilder::new().build();
        assert!(ken.is_bagage_ok());
    }
    #[test]
    fn w_and_b_nok() {
        let ken = KenBuilder::new()
            .pic(70.0)
            .pax_front(80.0)
            .pax_left_back(80.0)
            .bagage(23.0)
            .fuel(129.0)
            .build();
        assert!(!ken.is_weight_and_balance_ok());
    }
}
