use crate::ViktArm;

#[derive(Debug, Clone)]
pub struct FourSeater {
    pub base_weight: ViktArm,
    pub fuel: ViktArm,
    pub pic: ViktArm,
    pub pax_front: ViktArm,
    pub pax_left_back: ViktArm,
    pub pax_right_back: ViktArm,
}

impl FourSeater {
    pub fn sum_weight(&self) -> f32 {
        self.base_weight.weight
            + self.fuel.weight
            + self.pic.weight
            + self.pax_front.weight
            + self.pax_left_back.weight
            + self.pax_right_back.weight
    }
    pub fn sum_torque(&self) -> f32 {
        self.base_weight.torque()
            + self.fuel.torque()
            + self.pic.torque()
            + self.pax_front.torque()
            + self.pax_left_back.torque()
            + self.pax_right_back.torque()
    }
}

#[derive(Default)]
pub struct FourSeaterBuilder {
    base_weight: ViktArm,
    fuel: ViktArm,
    pic: ViktArm,
    pax_front: ViktArm,
    pax_left_back: ViktArm,
    pax_right_back: ViktArm,
}

impl FourSeaterBuilder {
    pub fn fuel(&mut self, fuel: ViktArm) -> &FourSeaterBuilder {
        self.fuel = fuel;
        self
    }
    pub fn pic(&mut self, w_pic: ViktArm) -> &FourSeaterBuilder {
        self.pic = w_pic;
        self
    }
    pub fn pax_front(&mut self, pax: ViktArm) -> &FourSeaterBuilder {
        self.pax_front = pax;
        self
    }
    pub fn pax_left_back(&mut self, pax: ViktArm) -> &FourSeaterBuilder {
        self.pax_left_back = pax;
        self
    }
    pub fn pax_right_back(&mut self, pax: ViktArm) -> &FourSeaterBuilder {
        self.pax_right_back = pax;
        self
    }
    pub fn base_weight(&mut self, base_weight: ViktArm) -> &FourSeaterBuilder {
        self.base_weight = base_weight;
        self
    }

    pub fn build(self) -> FourSeater {
        FourSeater {
            base_weight: self.base_weight,
            fuel: self.fuel,
            pic: self.pic,
            pax_front: self.pax_front,
            pax_left_back: self.pax_left_back,
            pax_right_back: self.pax_right_back,
        }
    }
}
