use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    calc_wb::WeightAndBalance, is_inside_polygon, is_value_within_weight_limit, FailReason, Kind,
    WeightLever,
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Input {
    pub name: String,
    pub values: HashMap<String, serde_json::Value>,
}

pub struct ParsedInput {
    pub name: String,
    pub values: HashMap<Kind, f32>,
}

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
    pub max_fuel_weight: f32,
    pub max_zero_fuel_mass: Option<f32>,
    pub max_bagage_weight: Option<f32>,
    pub max_bagage_weight_front: Option<f32>,
    pub max_bagage_weight_back: Option<f32>,
    pub max_bagage_weight_wings: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PlaneData {
    pub name: String,
    pub levers: Levers,
    pub max_weights: MaxWeights,
    pub vertices: [[f32; 2]; 6],
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

    fn is_mtow_ok(&self, prop: &PlaneProperties) -> Result<(), FailReason> {
        if prop.get_total_weights() > self.max_weights.max_take_off_weight {
            return Err(FailReason::MaxTakeOffWeight);
        }
        Ok(())
    }

    fn flatten_vertices(&self) -> [WeightLever; 6] {
        self.vertices
            .iter()
            .map(|vertex| WeightLever::new(vertex[0], vertex[1]))
            .collect::<Vec<WeightLever>>()
            .try_into()
            .expect("Should be able to create array")
    }

    fn is_zero_fuel_ok(&self, prop: &PlaneProperties) -> Result<(), FailReason> {
        let (total_weight, total_torque) = prop
            .0
            .iter()
            .filter(|(kind, _)| **kind != Kind::Fuel || **kind != Kind::TripFuel)
            .fold((0.0_f32, 0.0_f32), |acc, (_, wb)| {
                (acc.0 + wb.weight, acc.1 + wb.torque())
            });
        let zero_fuel_point = WeightLever::new(total_weight, total_torque / total_weight);
        if let Err(_) = is_inside_polygon(zero_fuel_point, &self.flatten_vertices(), false) {
            return Err(FailReason::ZeroFuel);
        }
        Ok(())
    }

    fn is_bagage_ok(&self, prop: &PlaneProperties) -> Result<(), FailReason> {
        if !(self
            .max_weights
            .max_bagage_weight
            .and_then(|weight| Some(is_value_within_weight_limit(&prop.0, Kind::Bagage, weight)))
            .unwrap_or(true))
        {
            return Err(FailReason::Bagage);
        }
        if !(self
            .max_weights
            .max_bagage_weight_back
            .and_then(|weight| {
                Some(is_value_within_weight_limit(
                    &prop.0,
                    Kind::BagageBack,
                    weight,
                ))
            })
            .unwrap_or(true))
        {
            return Err(FailReason::BagageBack);
        }
        if !(self
            .max_weights
            .max_bagage_weight_front
            .and_then(|weight| {
                Some(is_value_within_weight_limit(
                    &prop.0,
                    Kind::BagageFront,
                    weight,
                ))
            })
            .unwrap_or(true))
        {
            return Err(FailReason::BagageFront);
        }
        Ok(())
    }

    fn is_bagage_in_wings_ok(&self, prop: &PlaneProperties) -> Result<(), FailReason> {
        if let Some(max_weight_wings) = self.max_weights.max_bagage_weight_wings {
            if prop
                .0
                .get(&Kind::BagageWings)
                .and_then(|wings| Some(wings.weight > max_weight_wings))
                .expect("Config contains bagage in wings but missing in input.")
            {
                return Err(FailReason::BagageWings);
            }
        }
        Ok(())
    }

    fn is_max_wing_load_ok(&self, properties: &PlaneProperties) -> Result<(), FailReason> {
        if let Some(max_weight) = self.max_weights.max_zero_fuel_mass {
            let properties_of_interest = [
                Kind::Base,
                Kind::Pilot,
                Kind::CoPilot,
                Kind::BagageBack,
                Kind::BagageFront,
            ];
            if properties
                .0
                .iter()
                .filter(|(k, _)| properties_of_interest.contains(k))
                .map(|(_, wb)| wb.weight)
                .sum::<f32>()
                > max_weight
            {
                return Err(FailReason::MaxWingLoad);
            }
        }
        Ok(())
    }

    fn is_fuel_weight_ok(&self, properties: &PlaneProperties) -> Result<(), FailReason> {
        if !(properties
            .0
            .get(&Kind::Fuel)
            .and_then(|fuel| Some(fuel.weight <= self.max_weights.max_fuel_weight))
            .unwrap_or(true))
        {
            return Err(FailReason::Fuel);
        }
        Ok(())
    }

    fn is_landing_fuel_ok(&self, properties: &PlaneProperties) -> Result<(), FailReason> {
        if !(properties
            .0
            .get(&Kind::TripFuel)
            .and_then(|fuel| {
                Some(fuel.weight > 0.0 && fuel.weight <= self.max_weights.max_fuel_weight)
            })
            .unwrap_or(true))
        {
            return Err(FailReason::LandingFuel);
        }
        Ok(())
    }

    fn check_limits(&self, prop: &PlaneProperties) -> Result<(), FailReason> {
        self.is_mtow_ok(prop)?;
        self.is_max_wing_load_ok(prop)?;
        self.is_bagage_in_wings_ok(prop)?;
        self.is_bagage_ok(prop)?;
        self.is_fuel_weight_ok(prop)?;
        self.is_zero_fuel_ok(prop)?;
        self.is_landing_fuel_ok(prop)?;
        self.is_zero_fuel_ok(prop)?;

        Ok(())
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

    fn is_weight_and_balance_ok(&self, prop: &PlaneProperties) -> Result<(), FailReason> {
        self.check_limits(prop)?;
        let calc = self.calc_weight_and_balance(prop);
        is_inside_polygon(calc, &self.flatten_vertices(), false)?;

        Ok(())
    }

    fn is_landing_weight_and_balance_ok(&self, prop: &PlaneProperties) -> Result<(), FailReason> {
        self.check_limits(prop)?;

        let calc = self.calc_landing_weight_and_balance(prop);
        is_inside_polygon(calc, &self.flatten_vertices(), false)?;
        Ok(())
    }
}
