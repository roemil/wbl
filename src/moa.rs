use std::fmt::format;

use crate::{
    calc_wb::CalcWeightAndBalance, is_inside_polygon, is_value_within_weight_limit, Kind, MoaJson, ViktArm
};

type Properties = std::collections::HashMap<Kind, ViktArm>;
type Verticies = [ViktArm; 6];
#[derive(Debug, Clone)]
pub struct Moa {
    properties: Properties,
    vertices: [ViktArm; 6],
}

impl Default for Moa {
    fn default() -> Self {
        Self { properties: Default::default(), vertices: Default::default() }
    }
}

impl Moa {
    pub fn new(properties: Properties, vertices: Verticies) -> Moa {
        Moa {
            properties,
            vertices,
        }
    }
    fn is_max_wing_load_ok(&self) -> bool {
        let properties_of_interest = [
            Kind::Base,
            Kind::Pilot,
            Kind::CoPilot,
            Kind::BagageBack,
            Kind::BagageFront,
        ];
        self.properties
            .iter()
            .filter(|(k, _)| properties_of_interest.contains(k))
            .map(|(_, wb)| wb.weight)
            .sum::<f32>()
            <= 660.0
    }

    fn is_mtow_ok(&self) -> bool {
        self.properties.iter().map(|(_, wb)| wb.weight).sum::<f32>() <= 750.0
    }

    fn is_zero_fuel_ok(&self) -> bool {
        let (total_weight, total_torque) = self
            .properties
            .iter()
            .filter(|(kind, _)| **kind != Kind::Fuel)
            .fold((0.0_f32, 0.0_f32), |acc, (_, wb)| {
                (acc.0 + wb.weight, acc.1 + wb.torque())
            });
        let zero_fuel_point = ViktArm::new(total_weight, total_torque / total_weight);
        is_inside_polygon(zero_fuel_point, &self.vertices, false)
    }

    fn is_bagage_ok(&self) -> bool {
        is_value_within_weight_limit(&self.properties, Kind::BagageBack, 15.0)
            && is_value_within_weight_limit(&self.properties, Kind::BagageFront, 1.0)
    }

    fn is_bagage_in_wings_ok(&self) -> bool {
        let mut is_bagage_wings_is_ok = true;
        if let Some(bagage_wings) = self.properties.get(&Kind::BagageWings) {
            is_bagage_wings_is_ok = bagage_wings.weight <= 40.0;
        }
        is_bagage_wings_is_ok
    }

    fn get_total_weights(&self) -> f32 {
        self.properties.iter().map(|(_, wb)| wb.weight).sum()
    }
    fn get_total_torque(&self) -> f32 {
        self.properties
            .iter()
            .map(|(_, wb)| wb.torque())
            .sum::<f32>()
    }
    pub fn update_weight(&mut self, kind : Kind, weight : f32) -> Result<(), String>{
        if let Some(arm) = self.properties.get_mut(&kind) {
            arm.weight = weight;
            return Ok(());
        }
        Err(format!("Key ({:?}) not present in MOA", kind).to_string())
    }
}

impl CalcWeightAndBalance for Moa {
    fn calc_weight_and_balance(&self) -> ViktArm {
        let total_weight = self.get_total_weights();
        ViktArm {
            weight: total_weight,
            lever: self.get_total_torque() / total_weight,
        }
    }

    fn is_weight_and_balance_ok(&self) -> bool {
        if !self.is_mtow_ok()
            || !self.is_max_wing_load_ok()
            || !self.is_bagage_in_wings_ok()
            || !self.is_bagage_ok()
        {
            return false;
        }

        let calc = self.calc_weight_and_balance();
        is_inside_polygon(calc, &self.vertices, false) && self.is_zero_fuel_ok()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn bagage_ok_no_bagage() {
    //     let moa = MoaBuilder::new().build();
    //     assert!(moa.is_bagage_ok());
    // }
    // #[test]
    // fn bagage_ok_back_bagage() {
    //     let moa = MoaBuilder::new().bagage_back(10.0).build();
    //     assert!(moa.is_bagage_ok());
    // }
    // #[test]
    // fn bagage_ok_front_bagage() {
    //     let moa = MoaBuilder::new().bagage_front(0.5).build();
    //     assert!(moa.is_bagage_ok());
    // }
    // #[test]
    // fn bagage_ok_both_bagage() {
    //     let moa = MoaBuilder::new()
    //         .bagage_back(10.0)
    //         .bagage_front(0.5)
    //         .build();
    //     assert!(moa.is_bagage_ok());
    // }
    // #[test]
    // fn bagage_nok_back_bagage() {
    //     let moa = MoaBuilder::new().bagage_back(41.0).build();
    //     assert!(!moa.is_bagage_ok());
    // }
    // #[test]
    // fn bagage_nok_front_bagage() {
    //     let moa = MoaBuilder::new().bagage_front(1.5).build();
    //     assert!(!moa.is_bagage_ok());
    // }
    // #[test]
    // fn bagage_nok_both_bagage() {
    //     let moa = MoaBuilder::new()
    //         .bagage_back(41.0)
    //         .bagage_front(1.5)
    //         .build();
    //     assert!(!moa.is_bagage_ok());
    // }
}
