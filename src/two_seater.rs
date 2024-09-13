use crate::{UseFuel, ViktArm};

#[derive(Debug, Clone)]
pub struct TwoSeater {
    pub base_weight: ViktArm,
    pub fuel: ViktArm,
    pub w_pic: ViktArm,
    pub w_pax: ViktArm,
}

impl TwoSeater {
    pub fn sum_weight(&self, use_fuel: UseFuel) -> f32 {
        if use_fuel == UseFuel::Yes {
            self.base_weight.weight + self.fuel.weight + self.w_pic.weight + self.w_pax.weight
        } else {
            self.base_weight.weight + self.w_pic.weight + self.w_pax.weight
        }
    }
    pub fn sum_torque(&self, use_fuel: UseFuel) -> f32 {
        if use_fuel == UseFuel::Yes {
            self.base_weight.torque()
                + self.fuel.torque()
                + self.w_pic.torque()
                + self.w_pax.torque()
        } else {
            self.base_weight.torque() + self.w_pic.torque() + self.w_pax.torque()
        }
    }
}

pub struct TwoSeaterBuilder {
    base_weight: ViktArm,
    fuel: ViktArm,
    w_pic: ViktArm,
    w_pax: ViktArm,
}

impl std::default::Default for TwoSeaterBuilder {
    fn default() -> TwoSeaterBuilder {
        TwoSeaterBuilder {
            base_weight: ViktArm::new(0.0, 0.0),
            fuel: ViktArm::new(0.0, 0.0),
            w_pic: ViktArm::new(0.0, 0.0),
            w_pax: ViktArm::new(0.0, 0.0),
        }
    }
}

impl TwoSeaterBuilder {
    pub fn fuel(&mut self, fuel: ViktArm) -> &TwoSeaterBuilder {
        self.fuel = fuel;
        self
    }
    pub fn pic(&mut self, w_pic: ViktArm) -> &TwoSeaterBuilder {
        self.w_pic = w_pic;
        self
    }
    pub fn pax(&mut self, pax: ViktArm) -> &TwoSeaterBuilder {
        self.w_pax = pax;
        self
    }
    pub fn base_weight(&mut self, base_weight: ViktArm) -> &TwoSeaterBuilder {
        self.base_weight = base_weight;
        self
    }

    pub fn build(self) -> TwoSeater {
        TwoSeater {
            base_weight: self.base_weight,
            fuel: self.fuel,
            w_pic: self.w_pic,
            w_pax: self.w_pax,
        }
    }
}
