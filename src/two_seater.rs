use crate::{UseFuel, ViktArm};

#[derive(Debug, Clone)]
pub struct TwoSeater {
    pub base_weight: ViktArm,
    pub fuel: ViktArm,
    pub pic: ViktArm,
    pub pax: ViktArm,
}

impl TwoSeater {
    pub fn sum_weight(&self, use_fuel: UseFuel) -> f32 {
        if use_fuel == UseFuel::Yes {
            self.base_weight.weight + self.fuel.weight + self.pic.weight + self.pax.weight
        } else {
            self.base_weight.weight + self.pic.weight + self.pax.weight
        }
    }
    pub fn sum_torque(&self, use_fuel: UseFuel) -> f32 {
        if use_fuel == UseFuel::Yes {
            self.base_weight.torque()
                + self.fuel.torque()
                + self.pic.torque()
                + self.pax.torque()
        } else {
            self.base_weight.torque() + self.pic.torque() + self.pax.torque()
        }
    }
}

pub struct TwoSeaterBuilder {
    base_weight: ViktArm,
    fuel: ViktArm,
    pic: ViktArm,
    pax: ViktArm,
}

impl std::default::Default for TwoSeaterBuilder {
    fn default() -> TwoSeaterBuilder {
        TwoSeaterBuilder {
            base_weight: ViktArm::new(0.0, 0.0),
            fuel: ViktArm::new(0.0, 0.0),
            pic: ViktArm::new(0.0, 0.0),
            pax: ViktArm::new(0.0, 0.0),
        }
    }
}

impl TwoSeaterBuilder {
    pub fn fuel(&mut self, fuel: ViktArm) -> &TwoSeaterBuilder {
        self.fuel = fuel;
        self
    }
    pub fn pic(&mut self, w_pic: ViktArm) -> &TwoSeaterBuilder {
        self.pic = w_pic;
        self
    }
    pub fn pax(&mut self, pax: ViktArm) -> &TwoSeaterBuilder {
        self.pax = pax;
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
            pic: self.pic,
            pax: self.pax,
        }
    }
}
