use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    calc_wb::WeightAndBalance, is_inside_polygon, is_value_within_weight_limit, Kind, WeightLever,
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Levers {
    pub base: f32,
    pub fuel: f32,
    pub trip_fuel: f32,
    pub bagage: Option<f32>,
    pub bagage_back: Option<f32>,
    pub bagage_front: Option<f32>,
    pub bagage_wings: Option<f32>,
    pub pilot: f32,
    pub co_pilot: f32,
    pub passenger_left: Option<f32>,
    pub passenger_right: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MaxWeights {
    pub max_take_off_weight: f32,
    pub max_fuel: f32,
    pub max_zero_fuel_mass: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PlaneData {
    pub name: String,
    pub levers: Levers,
    pub max_weights: MaxWeights,
    pub vortices: [[f32; 2]; 6],
}

impl PlaneData {
    pub fn to_lever_map(&self) -> HashMap<Kind, f32> {
        let mut map = HashMap::new();
        map.insert(Kind::Base, self.levers.base);
        map.insert(Kind::Fuel, self.levers.fuel);
        map.insert(Kind::TripFuel, self.levers.trip_fuel);
        if let Some(value) = self.levers.bagage_back {
            map.insert(Kind::BagageBack, value);
        }
        if let Some(value) = self.levers.bagage_front {
            map.insert(Kind::BagageFront, value);
        }
        if let Some(value) = self.levers.bagage_wings {
            map.insert(Kind::BagageWings, value);
        }
        if let Some(value) = self.levers.bagage {
            map.insert(Kind::Bagage, value);
        }

        map.insert(Kind::Pilot, self.levers.pilot);
        map.insert(Kind::CoPilot, self.levers.co_pilot);

        if let Some(value) = self.levers.passenger_left {
            map.insert(Kind::PaxLeftBack, value);
        }
        if let Some(value) = self.levers.passenger_right {
            map.insert(Kind::PaxRightBack, value);
        }

        map
    }

    fn is_mtow_ok(&self, prop: &PlaneProperties) -> bool {
        prop.get_total_weights() <= self.max_weights.max_take_off_weight
    }

    fn flatten_vortices(&self) -> [WeightLever; 6] {
        self.vortices
            .iter()
            .map(|vortex| WeightLever::new(vortex[0], vortex[1]))
            .collect::<Vec<WeightLever>>()
            .try_into()
            .expect("Should be able to create array")
    }

    fn is_zero_fuel_ok(&self, prop: &PlaneProperties) -> bool {
        let (total_weight, total_torque) = prop
            .0
            .iter()
            .filter(|(kind, _)| **kind != Kind::Fuel)
            .fold((0.0_f32, 0.0_f32), |acc, (_, wb)| {
                (acc.0 + wb.weight, acc.1 + wb.torque())
            });
        let zero_fuel_point = WeightLever::new(total_weight, total_torque / total_weight);
        is_inside_polygon(zero_fuel_point, &self.flatten_vortices(), false)
    }

    fn is_bagage_ok(&self, prop: &PlaneProperties) -> bool {
        let mut is_bagage_ok = true;
        if self.levers.bagage.is_some() {
            is_bagage_ok =
                is_value_within_weight_limit(&prop.0, Kind::Bagage, self.levers.bagage.unwrap());
        }
        let mut is_bagage_back_ok = true;
        if self.levers.bagage_back.is_some() {
            is_bagage_back_ok = is_value_within_weight_limit(
                &prop.0,
                Kind::BagageBack,
                self.levers.bagage_back.unwrap(),
            );
        }
        let mut is_bagage_front_ok = true;
        if self.levers.bagage_front.is_some() {
            is_bagage_front_ok = is_value_within_weight_limit(
                &prop.0,
                Kind::BagageFront,
                self.levers.bagage_front.unwrap(),
            );
        }
        is_bagage_ok && is_bagage_back_ok && is_bagage_front_ok
    }

    fn is_bagage_in_wings_ok(&self, prop: &PlaneProperties) -> bool {
        let mut is_bagage_wings_is_ok = true;
        if let Some(bagage_wings) = prop.0.get(&Kind::BagageWings) {
            is_bagage_wings_is_ok = bagage_wings.weight
                <= self
                    .levers
                    .bagage_wings
                    .expect("Config is missing Bagage in wings");
        }
        is_bagage_wings_is_ok
    }

    fn is_max_wing_load_ok(&self, properties: &PlaneProperties) -> bool {
        if let Some(max_weight) = self.max_weights.max_zero_fuel_mass {
            let properties_of_interest = [
                Kind::Base,
                Kind::Pilot,
                Kind::CoPilot,
                Kind::BagageBack,
                Kind::BagageFront,
            ];
            return properties
                .0
                .iter()
                .filter(|(k, _)| properties_of_interest.contains(k))
                .map(|(_, wb)| wb.weight)
                .sum::<f32>()
                <= max_weight;
        }
        true
    }

    fn is_fuel_ok(&self, properties: &PlaneProperties) -> bool {
        if let Some(fuel) = properties.0.get(&Kind::Fuel) {
            return fuel.weight <= self.max_weights.max_fuel;
        }
        true
    }
    fn is_landing_fuel_ok(&self, properties: &PlaneProperties) -> bool {
        if let Some(fuel) = properties.0.get(&Kind::TripFuel) {
            return fuel.weight > 0.0
                && fuel.weight <= self.max_weights.max_fuel;
        }
        true
    }
}

#[derive(Default)]
pub struct PlaneProperties(HashMap<Kind, WeightLever>);

impl PlaneProperties {
    pub fn new(val: HashMap<Kind, WeightLever>) -> PlaneProperties {
        PlaneProperties(val)
    }
    fn get_total_weights(&self) -> f32 {
        self.0
            .iter()
            .filter(|(k, _)| **k != Kind::TripFuel)
            .map(|(_, v)| v.weight)
            .sum()
    }

    fn get_landing_weights(&self) -> f32 {
        self.0
            .iter()
            .filter(|(k, _)| **k != Kind::TripFuel)
            .map(|(_, v)| v.weight)
            .sum::<f32>()
            - self
                .0
                .get(&Kind::TripFuel)
                .expect("Missing Trip fuel")
                .weight
    }

    fn get_total_torque(&self) -> f32 {
        self.0
            .iter()
            .filter(|(k, _)| **k != Kind::TripFuel)
            .map(|(_, v)| v.torque())
            .sum()
    }

    fn get_landing_torque(&self) -> f32 {
        self.0
            .iter()
            .filter(|(k, _)| **k != Kind::TripFuel)
            .map(|(_, v)| v.torque())
            .sum::<f32>()
            - self
                .0
                .get(&Kind::TripFuel)
                .expect("Missing Trip fuel")
                .torque()
    }
}

impl WeightAndBalance for PlaneData {
    fn calc_weight_and_balance(&self, prop: &PlaneProperties) -> WeightLever {
        let total_weight = prop.get_total_weights();
        assert!(total_weight > 0.0);
        WeightLever {
            weight: total_weight,
            lever: prop.get_total_torque() / total_weight,
        }
    }

    fn calc_landing_weight_and_balance(&self, prop: &PlaneProperties) -> WeightLever {
        let total_weight = prop.get_landing_weights();
        assert!(total_weight > 0.0);
        WeightLever {
            weight: total_weight,
            lever: prop.get_landing_torque() / total_weight,
        }
    }

    fn is_weight_and_balance_ok(&self, prop: &PlaneProperties) -> bool {
        if !self.is_mtow_ok(prop)
            || !self.is_max_wing_load_ok(prop)
            || !self.is_bagage_in_wings_ok(prop)
            || !self.is_bagage_ok(prop)
            || !self.is_fuel_ok(prop)
            || !self.is_zero_fuel_ok(prop)
            || !self.is_landing_fuel_ok(prop)
        {
            println!("limits failed");
            return false;
        }

        let calc = self.calc_weight_and_balance(prop);
        is_inside_polygon(calc, &self.flatten_vortices(), false) && self.is_zero_fuel_ok(prop)
    }

    fn is_landing_weight_and_balance_ok(&self, prop: &PlaneProperties) -> bool {
        if !self.is_mtow_ok(prop)
            || !self.is_max_wing_load_ok(prop)
            || !self.is_bagage_in_wings_ok(prop)
            || !self.is_bagage_ok(prop)
            || !self.is_fuel_ok(prop)
            || !self.is_zero_fuel_ok(prop)
            || !self.is_landing_fuel_ok(prop)
        {
            println!("limits failed");
            return false;
        }

        let calc = self.calc_landing_weight_and_balance(prop);
        is_inside_polygon(calc, &self.flatten_vortices(), false) && self.is_zero_fuel_ok(prop)
    }
}
