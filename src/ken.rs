use crate::{
    calc_wb::CalcWeightAndBalance, is_inside_polygon, Kind, ViktArm
};

pub struct Ken {
    properties: std::collections::HashMap<Kind, ViktArm>,
}

#[derive(Default)]
pub struct KenBuilder {
    properties: std::collections::HashMap<Kind, ViktArm>,
}

impl KenBuilder {
    pub fn new() -> KenBuilder {
       let mut properties = std::collections::HashMap::<Kind, ViktArm>::default();
       properties.insert(Kind::Base, ViktArm::new(685.2, 219.4));
        KenBuilder { properties }
    }

    pub fn fuel(mut self, fuel: f32) -> KenBuilder {
        self.properties
            .insert(Kind::Fuel, ViktArm::new(fuel, 241.3));
        self
    }
    pub fn bagage(mut self, bagage: f32) -> KenBuilder {
        self.properties
            .insert(Kind::Bagage, ViktArm::new(bagage, 362.7));
        self
    }
    pub fn pic(mut self, w_pic: f32) -> KenBuilder {
        self.properties
            .insert(Kind::Pilot, ViktArm::new(w_pic, 204.4));
        self
    }
    pub fn copilot(mut self, pax: f32) -> KenBuilder {
        self.properties
            .insert(Kind::CoPilot, ViktArm::new(pax, 204.4));
        self
    }
    pub fn pax_left_back(mut self, pax: f32) -> KenBuilder {
        self.properties
            .insert(Kind::PaxLeftBack, ViktArm::new(pax, 300.0));
        self
    }
    pub fn pax_right_back(mut self, pax: f32) -> KenBuilder {
        self.properties
            .insert(Kind::PaxRightBack, ViktArm::new(pax, 300.0));
        self
    }

    pub fn build(self) -> Ken {
        Ken {
            properties: self.properties,
        }
    }
}

impl Ken {
    fn is_mtow_ok(&self) -> bool {
        self.calc_weight_and_balance().weight <= 1055.0
    }

    fn is_bagage_ok(&self) -> bool {
        if let Some(bagage) = self.properties.get(&Kind::Bagage) {
            return bagage.weight <= 15.0;
        }
        true
    }
    fn is_fuel_ok(&self) -> bool {
        if let Some(fuel) = self.properties.get(&Kind::Fuel) {
            return fuel.weight <= 15.0;
        }
        true
    }
}

impl CalcWeightAndBalance for Ken {
    fn calc_weight_and_balance(&self) -> ViktArm {
        let total_w = self
            .properties
            .iter()
            .map(|(_, wb)| wb.weight)
            .sum();
        assert!(total_w > 0.0);

        let total_torque = self
            .properties
            .iter()
            .map(|(_, wb)| wb.torque())
            .sum::<f32>();

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
        let calc = self.calc_weight_and_balance();
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
            .copilot(80.0)
            .pax_left_back(80.0)
            .bagage(23.0)
            .fuel(129.0)
            .build();
        assert!(!ken.is_weight_and_balance_ok());
    }
}
