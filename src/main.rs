use wbl::{calc_wb::CalcWeightAndBalance, moa::MoaBuilder};

fn main() {
    let moa = MoaBuilder::new()
        .fuel(85.0)
        .pic(70.0)
        .bagage_wings(40.0)
        .build();
    let moa_calc = moa.clone().calc_wb();
    println!(
        "Moa lever: {}, Moa weight: {}",
        moa_calc.lever, moa_calc.weight
    );
    println!("Moa is ok: {}", moa.is_weight_and_balance_ok());
}
