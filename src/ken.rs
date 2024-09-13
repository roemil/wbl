use crate::{calc_wb::CalcWeightAndBalance, is_inside_polygon, ViktArm};

#[derive(Debug, Clone)]
pub struct Ken {
    base_weight: ViktArm,
    fuel: ViktArm,
    bagage: ViktArm,
    w_pic: ViktArm,
    w_pax_front: ViktArm,
    w_pax_left_back: ViktArm,
    w_pax_right_back: ViktArm,
}
pub struct KenBuilder {
    fuel: f32,
    bagage: f32,
    w_pic: f32,
    w_pax_front: f32,
    w_pax_left_back: f32,
    w_pax_right_back: f32,
}

impl KenBuilder {
    pub fn new() -> KenBuilder {
        KenBuilder {
            fuel: 0.0,
            bagage: 0.0,
            w_pic: 0.0,
            w_pax_front: 0.0,
            w_pax_left_back: 0.0,
            w_pax_right_back: 0.0,
        }
    }

    pub fn fuel(mut self, fuel: f32) -> KenBuilder {
        self.fuel = fuel;
        self
    }
    pub fn bagage(mut self, bagage: f32) -> KenBuilder {
        self.bagage = bagage;
        self
    }
    pub fn pic(mut self, w_pic: f32) -> KenBuilder {
        self.w_pic = w_pic;
        self
    }
    pub fn pax_front(mut self, pax: f32) -> KenBuilder {
        self.w_pax_front = pax;
        self
    }
    pub fn pax_left_back(mut self, pax: f32) -> KenBuilder {
        self.w_pax_left_back = pax;
        self
    }
    pub fn pax_right_back(mut self, pax: f32) -> KenBuilder {
        self.w_pax_right_back = pax;
        self
    }

    pub fn build(self) -> Ken {
        Ken {
            base_weight: ViktArm::new(685.2, 219.4),
            fuel: ViktArm::new(0.0, 241.3),
            bagage: ViktArm::new(0.0, 362.7),
            w_pic: ViktArm::new(0.0, 204.4),
            w_pax_front: ViktArm::new(0.0, 204.4),
            w_pax_left_back: ViktArm::new(0.0, 300.0),
            w_pax_right_back: ViktArm::new(0.0, 300.0),
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
        self.fuel.weight <= 129.0
    }
}

impl CalcWeightAndBalance for Ken {
    fn calc_wb(&self) -> ViktArm {
        let total_w = self.base_weight.weight
            + self.fuel.weight
            + self.bagage.weight
            + self.w_pic.weight
            + self.w_pax_front.weight
            + self.w_pax_left_back.weight
            + self.w_pax_right_back.weight;
        assert!(total_w > 0.0);

        let total_torque = self.base_weight.torque()
            + self.fuel.torque()
            + self.bagage.torque()
            + self.w_pic.torque()
            + self.w_pax_front.torque()
            + self.w_pax_left_back.torque()
            + self.w_pax_right_back.torque();

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
