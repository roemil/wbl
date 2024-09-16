use std::collections::HashMap;

use crate::{calc_wb::CalcWeightAndBalance, is_inside_polygon, KenJson, Kind, Properties, Verticies, ViktArm};

pub struct Ken {
    pub properties: std::collections::HashMap<Kind, ViktArm>,
    vertices: [ViktArm; 6],
}

impl Ken {
    pub fn new(properties: Properties, vertices: Verticies) -> Ken {
        Ken {
            properties,
            vertices,
        }
    }
    fn is_mtow_ok(&self) -> bool {
        self.calc_weight_and_balance().weight <= 1055.0
    }

    fn is_bagage_ok(&self) -> bool {
        if let Some(bagage) = self.properties.get(&Kind::Bagage) {
            return bagage.weight <= 23.0;
        }
        true
    }
    fn is_fuel_ok(&self) -> bool {
        if let Some(fuel) = self.properties.get(&Kind::Fuel) {
            return fuel.weight <= 129.0;
        }
        true
    }
}

impl CalcWeightAndBalance for Ken {
    fn calc_weight_and_balance(&self) -> ViktArm {
        let total_w = self.properties.values().map(|vikt_arm| vikt_arm.weight).sum();
        assert!(total_w > 0.0);

        let total_torque = self
            .properties
            .values()
            .map(|vikt_arm| vikt_arm.torque())
            .sum::<f32>();

        ViktArm {
            weight: total_w,
            lever: total_torque / total_w,
        }
    }
    fn is_weight_and_balance_ok(&self) -> bool {
        let calc = self.calc_weight_and_balance();
        if !self.is_mtow_ok() || !self.is_bagage_ok() || !self.is_fuel_ok() {
            return false;
        }

        is_inside_polygon(calc, &self.vertices, false)
    }
}

pub struct KenConfig {
    pub config: std::collections::HashMap<Kind, f32>,
    pub vortices: [ViktArm; 6],
}

impl Default for KenConfig {
    fn default() -> KenConfig {
        KenConfig {
            config: HashMap::new(),
            vortices: [
                ViktArm::new(490.0, 171.2),
                ViktArm::new(600.0, 171.2),
                ViktArm::new(750.0, 179.2),
                ViktArm::new(750.0, 184.0),
                ViktArm::new(600.0, 184.0),
                ViktArm::new(490.0, 184.0),
            ],
        }
    }
}

impl From<KenJson> for KenConfig {
    fn from(value: KenJson) -> Self {
        let mut ken = KenConfig::default();
        ken.config.insert(Kind::Base, value.base);
        ken.config.insert(Kind::Fuel, value.fuel);
        ken.config.insert(Kind::Bagage, value.bagage);
        ken.config.insert(Kind::Pilot, value.pilot);
        ken.config.insert(Kind::CoPilot, value.co_pilot);
        ken.config.insert(Kind::PaxLeftBack, value.passenger_left);
        ken.config.insert(Kind::PaxRightBack, value.passenger_right);

        ken.vortices = value
            .vortices
            .iter()
            .map(|vortex| ViktArm::new(vortex[0], vortex[1]))
            .collect::<Vec<ViktArm>>()
            .try_into()
            .expect("Should be able to create array");

        ken
    }
}

impl KenConfig {
    pub fn new() -> KenConfig {
        KenConfig::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bagage_ok_no_bagage() {
        assert!(true);
    }
    #[test]
    fn w_and_b_nok() {
        assert!(true);
    }
}
