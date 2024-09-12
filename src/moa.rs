use crate::{calc_wb::CalcWeightAndBalance, is_inside_polygon, ViktArm};

#[derive(Debug, Clone)]
pub struct Moa {
    tomvikt: ViktArm,
    fuel: ViktArm,
    bagage_back: ViktArm,
    bagage_front: ViktArm,
    bagage_wings: ViktArm,
    w_pic: ViktArm,
    w_pax: ViktArm,
}
pub struct MoaBuilder {
    fuel: f32,
    bagage_back: f32,
    bagage_front: f32,
    bagage_wings: f32,
    w_pic: f32,
    w_pax: f32,
}

impl MoaBuilder {
    pub fn new() -> MoaBuilder {
        MoaBuilder {
            fuel: 0.0,
            bagage_back: 0.0,
            bagage_front: 0.0,
            bagage_wings: 0.0,
            w_pic: 0.0,
            w_pax: 0.0,
        }
    }

    pub fn fuel(mut self, fuel: f32) -> MoaBuilder {
        self.fuel = fuel;
        self
    }
    pub fn bagage_back(mut self, bagage_back: f32) -> MoaBuilder {
        self.bagage_back = bagage_back;
        self
    }
    pub fn bagage_front(mut self, bagage_front: f32) -> MoaBuilder {
        self.bagage_front = bagage_front;
        self
    }
    pub fn bagage_wings(mut self, bagage_wings: f32) -> MoaBuilder {
        self.bagage_wings = bagage_wings;
        self
    }
    pub fn pic(mut self, w_pic: f32) -> MoaBuilder {
        self.w_pic = w_pic;
        self
    }
    pub fn pax(mut self, pax: f32) -> MoaBuilder {
        self.w_pax = pax;
        self
    }

    pub fn build(self) -> Moa {
        Moa {
            tomvikt: ViktArm::new(453.5, 172.9),
            fuel: ViktArm::new(self.fuel, 160.0),
            bagage_back: ViktArm::new(self.bagage_back, 252.0),
            bagage_front: ViktArm::new(self.bagage_front, 252.0),
            bagage_wings: ViktArm::new(self.bagage_wings, 202.5),
            w_pic: ViktArm::new(self.w_pic, 208.5),
            w_pax: ViktArm::new(self.w_pax, 208.5),
        }
    }
}

#[derive(PartialEq)]
enum UseFuel {
    Yes,
    No,
}

impl Moa {
    fn calc_wb_use_fuel(&self, use_fuel: UseFuel) -> ViktArm {
        let mut total_w = self.tomvikt.weight
            + self.bagage_back.weight
            + self.bagage_front.weight
            + self.bagage_wings.weight
            + self.w_pic.weight
            + self.w_pax.weight;
        if use_fuel == UseFuel::Yes {
            total_w += self.fuel.weight;
        }
        assert!(total_w > 0.0);
        let mut total_torque = self.tomvikt.torque()
            + self.bagage_back.torque()
            + self.bagage_front.torque()
            + self.bagage_wings.torque()
            + self.w_pic.torque()
            + self.w_pax.torque();

        if use_fuel == UseFuel::Yes {
            total_torque += self.fuel.torque()
        }

        ViktArm {
            weight: total_w,
            lever: total_torque / total_w,
        }
    }

    fn is_max_wing_load_ok(&self) -> bool {
        self.tomvikt.weight + self.w_pic.weight + self.w_pax.weight + self.bagage_back.weight
            <= 660.0
    }

    fn is_mtow_ok(&self) -> bool {
        self.calc_wb().weight <= 750.0
    }

    fn is_zero_fuel_ok(&self) -> bool {
        let zero_fuel_point = self.calc_wb_use_fuel(UseFuel::No);
        is_inside_polygon(zero_fuel_point, self.get_polygon(), false)
    }

    fn is_bagage_ok(&self) -> bool {
        self.bagage_back.weight <= 15.0 && self.bagage_front.weight <= 1.0
    }

    fn is_bagage_in_wings_ok(&self) -> bool {
        self.bagage_wings.weight <= 40.0
    }
}

impl CalcWeightAndBalance for Moa {
    fn calc_wb(&self) -> ViktArm {
        self.calc_wb_use_fuel(UseFuel::Yes)
    }
    fn get_polygon(&self) -> Vec<ViktArm> {
        vec![
            ViktArm::new(490.0, 171.2),
            ViktArm::new(600.0, 171.2),
            ViktArm::new(750.0, 179.2),
            ViktArm::new(750.0, 184.0),
            ViktArm::new(600.0, 184.0),
            ViktArm::new(490.0, 184.0),
        ]
    }
    fn is_weight_and_balance_ok(&self) -> bool {
        let calc = self.calc_wb();
        if !self.is_mtow_ok()
            || !self.is_max_wing_load_ok()
            || !self.is_bagage_in_wings_ok()
            || !self.is_bagage_ok()
        {
            return false;
        }
    
        let points = self.get_polygon();
    
        is_inside_polygon(calc, points, false) && self.is_zero_fuel_ok()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bagage_ok_no_bagage() {
        let moa = MoaBuilder::new().build();
        assert!(moa.is_bagage_ok());
    }
    #[test]
    fn bagage_ok_back_bagage() {
        let moa = MoaBuilder::new().bagage_back(10.0).build();
        assert!(moa.is_bagage_ok());
    }
    #[test]
    fn bagage_ok_front_bagage() {
        let moa = MoaBuilder::new().bagage_front(0.5).build();
        assert!(moa.is_bagage_ok());
    }
    #[test]
    fn bagage_ok_both_bagage() {
        let moa = MoaBuilder::new().bagage_back(10.0).bagage_front(0.5).build();
        assert!(moa.is_bagage_ok());
    }
    #[test]
    fn bagage_nok_back_bagage() {
        let moa = MoaBuilder::new().bagage_back(41.0).build();
        assert!(!moa.is_bagage_ok());
    }
    #[test]
    fn bagage_nok_front_bagage() {
        let moa = MoaBuilder::new().bagage_front(1.5).build();
        assert!(!moa.is_bagage_ok());
    }
    #[test]
    fn bagage_nok_both_bagage() {
        let moa = MoaBuilder::new().bagage_back(41.0).bagage_front(1.5).build();
        assert!(!moa.is_bagage_ok());
    }
}