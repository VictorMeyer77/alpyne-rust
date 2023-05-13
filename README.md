# alpyne-rust

### connection

|                  | Raspberry GPIO  | Motor Left  | Motor Right   | H Double Pont DC  | Breadboard |  Battery   |
|------------------|-----------------|-------------|---------------|-------------------|------------|------------|
| Dupont wire F/F  |       38        |             |               |        N1         |            |            |
| Dupont wire F/F  |       36        |             |               |        N2         |            |            |
| Dupont wire F/F  |       12        |             |               |        ENA        |            |            |
| Dupont wire F/F  |       18        |             |               |        N3         |            |            |
| Dupont wire F/F  |       16        |             |               |        N4         |            |            |
| Dupont wire F/F  |       35        |             |               |        ENB        |            |            |
| Dupont wire H/   |       2         |             |               |        5V         |            |            |
| Dupont wire M/F  |       4         |             |               |                   |    17 F    |            |
| Dupont wire      |       6         |             |               |       GND (Y)     |            |      -     |
| Dupont wire M/F  |       11        |             |               |                   |    16 F    |            |
| Dupont wire M/F  |       13        |             |               |                   |    15 C    |            |
| Dupont wire M/F  |       9         |             |               |                   |    5 B     |            |
| Dupont wire      |                 |     UP      |               |       OUT 4       |            |            |
| Dupont wire      |                 |    DOWN     |               |       OUT 3       |            |            |
| Dupont wire      |                 |             |      UP       |       OUT 1       |            |            |
| Dupont wire      |                 |             |     DOWN      |       OUT 2       |            |            |
| Dupont wire      |                 |             |               |       12V         |            |      +     |
| Dupont wire M/M  |                 |             |               |                   | 14 F - 5 E |            |
| Resistance       |                 |             |               |                   | 15 E - 15 G|            |
| Resistance       |                 |             |               |                   | 15 A - 5 A |            |
| HC-SR04          |                 |             |               |                   | 14I <-> 17I|            |

### gpio

![alt text](https://toptechboy.com/wp-content/uploads/2022/04/pinout-corrected.jpg)

### test

#### Run test

    cargo test -- --test-threads=1

#### Run test with coverage html report

prerequisites

    cargo +stable install cargo-llvm-cov --locked

run

     cargo llvm-cov --html
