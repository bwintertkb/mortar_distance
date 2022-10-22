# mortar_distance

A little library that can be used to callibrate the mortar firing distance in PlayerUnknown's Battlegrounds (PUBG).

### Video example

https://youtu.be/EApR86h8s4M

### Dependency

`mortar_distance = 0.1.0`

### Example

```rust
use mortar_distance::record;

const CALLIBRATION_DISTANCE: u32 = 100;

fn main() {
    println!("DISTANCE: {}", record(CALLIBRATION_DISTANCE, true).unwrap());
}
```

### License

`mortar_distance` is distributed under the terms of [MIT license](https://choosealicense.com/licenses/mit/).
