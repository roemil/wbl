use crate::{calc_wb::CalcWeightAndBalance, is_inside_polygon, ViktArm};

#[derive(Debug, Clone)]
pub struct Ken {
    tomvikt: ViktArm,
    fuel: ViktArm,
    bagage_back: ViktArm,
    bagage_front: ViktArm,
    w_pic: ViktArm,
    w_pax_front: ViktArm,
    w_pax_left_back: ViktArm,
    w_pax_right_back: ViktArm,
}
pub struct KenBuilder {
    fuel: f32,
    bagage_back: f32,
    bagage_front: f32,
    w_pic: f32,
    w_pax_front: f32,
    w_pax_left_back: f32,
    w_pax_right_back: f32,
}

impl KenBuilder {
    pub fn new() -> KenBuilder {
        KenBuilder {
            fuel: 0.0,
            bagage_back: 0.0,
            bagage_front: 0.0,
            w_pic: 0.0,
            w_pax_front: 0.0,
            w_pax_left_back: 0.0,
            w_pax_right_back: 0.0
        }
    }
    pub fn build(self) -> Ken {
        Ken {
            tomvikt: ViktArm::new(0.0, 0.0),
            fuel: ViktArm::new(0.0, 0.0),
            bagage_back: ViktArm::new(0.0, 0.0),
            bagage_front: ViktArm::new(0.0, 0.0),
            w_pic: ViktArm::new(0.0, 0.0),
            w_pax_front: ViktArm::new(0.0, 0.0),
            w_pax_left_back: ViktArm::new(0.0, 0.0),
            w_pax_right_back: ViktArm::new(0.0, 0.0),
        }
    }
}

impl Ken {
    fn is_mtow_ok(&self) -> bool {
        self.calc_wb().weight <= 750.0
    }

    fn is_bagage_ok(&self) -> bool {
        self.bagage_back.weight <= 15.0 && self.bagage_front.weight <= 1.0
    }
}


impl CalcWeightAndBalance for Ken {
    fn calc_wb(&self) -> ViktArm {
        todo!()
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
            || !self.is_bagage_ok()
        {
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
}
