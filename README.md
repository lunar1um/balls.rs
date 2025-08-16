## balls.rs
a balls bouncing simulation made in rust  
<br>
<img width="auto" height="auto" alt="image" src="https://github.com/user-attachments/assets/dfd9e4e7-f1dd-47eb-909b-bc2625c87779" />

#### controls:
- `LEFT` or `RIGHT` to change the `Force` of the Mouse when held down
- `UP` or `DOWN` to change the `Timescale` (aka the simulation speed)

#### current features:
- wall bounce based on window size
- balls collisions
- hold mouse down to apply force (pull / push) to every balls
- optional: changes colors on collisions
- optional: increases in size on collisions 

#### installation:
clone this repo

```shell
git clone https://github.com/lunar1um/balls.rs.git
cd balls.rs-main
```

build via cargo
```shell
cargo build --release
```
