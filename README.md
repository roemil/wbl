## Calculator for weight and balance for small airplanes
### Config
Add the configuration for each plane in src/config.json in the following format:
```json
{
    "name": "Name of airplane"
    "levers": {
        "base": lever for base wright,
        "fuel": lever for fuel weigh
    },
    "max_weights": {
        "max_take_off_weight" : some weight
        "max_fuel": max fuel in kg
    },
    "vertices" : [[ // 6 coordinate pairs
        x,
        y
    ]]
}
```
### Input
Currently the weight and balance is calculated by parsing a json file: src/input.json
Add your input weights in the following format:
```json
{
    "name": "Name of airplane",
    "base": "453.5",
    "fuel": "85.0",
    "bagage_back": "0.0",
    "bagage_front": "1.0",
    "bagage_wings": "2.0",
    "pilot": "70.0",
    "co_pilot": "0.0"
}
```

Weight should be in kilograms.

### Running
cargo run -- --path /path/to/input.json

### Output
Plane: "Your plane" has W&B that is ok: true
Plane: "Your plane" has W&B point at: ViktArm { weight: 611.5, lever: 175.40662 }