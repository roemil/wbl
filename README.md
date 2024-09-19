## Calculator for weight and balance for small airplanes
### Config
Add the configuration for each plane in src/input/config.json in the following format:
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
Levers are restricted to: 
- base (Mandatory)
- fuel (Mandatory)
- bagage
- bagage_back
- bagage_front
- bagage_wings
- pilot (Mandatory)
- co_pilot (Mandatory)
- passenger_left
- passenger_right

### Input
Currently the weight and balance is calculated by parsing a json file: src/input.json
Add your input weights in the following format:
```json
{
    "name": "Name of airplane",
    "values" {
        "base": "453.5",
        "fuel": "85.0",
        "bagage_back": "0.0",
        "bagage_front": "1.0",
        "bagage_wings": "2.0",
        "pilot": "70.0",
        "co_pilot": "0.0"
    }
}
```

Input points are restricted to: 
 - name (Mandatory)
 - base (Mandatory)
 - fuel (Mandatory)
 - bagage
 - bagage_back
 - bagage_front
 - bagage_wings
 - pilot (Mandatory)
 - co_pilot (Mandatory)
 - passenger_left
 - passenger_right

Weight should be in kilograms.

### Running
cargo run -- --path /path/to/input.json

### Output
Plane: "Your plane" has W&B that is ok: true
Plane: "Your plane" has W&B point at: ViktArm { weight: 611.5, lever: 175.40662 }

### Limitations
Calculations can be done on two- or four-seater planes.