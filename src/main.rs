use wbl::{calc_wb::CalcWeightAndBalance, ken::KenBuilder, moa::MoaBuilder};

fn main() {
    let moa = MoaBuilder::new()
        .fuel(85.0)
        .pic(70.0)
        .bagage_wings(40.0)
        .build();
    let moa_calc = moa.clone().calc_weight_and_balance();
    println!(
        "Moa lever: {}, Moa weight: {}",
        moa_calc.lever, moa_calc.weight
    );
    println!("Is MOA ok? {}", moa.is_weight_and_balance_ok());

    let ken = KenBuilder::new()
        .fuel(85.0)
        .pic(270.0)
        .pax_right_back(200.0)
        .build();

        println!("Is KEN ok? {}", ken.is_weight_and_balance_ok());
}
