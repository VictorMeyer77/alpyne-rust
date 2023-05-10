# alpyne-rust

### connection

|                  | Raspberry GPIO  | Motor Left  | Motor Right   | H Double Pont DC  | Breadboard |  Battery   |
|------------------|-----------------|-------------|---------------|-------------------|------------|------------|
| Dupont wire F/F  |       38        |             |               |        N1         |            |            |
| Dupont wire F/F  |       36        |             |               |        N2         |            |            |
| Dupont wire F/F  |       18        |             |               |        ENA        |            |            |
| Dupont wire F/F  |       18        |             |               |        N3         |            |            |
| Dupont wire F/F  |       16        |             |               |        N4         |            |            |
| Dupont wire F/F  |       35        |             |               |        ENB        |            |            |
| Dupont wire H/   |       2         |             |               |        5V         |            |            |
| Dupont wire M/F  |       4         |             |               |                   |    28 E    |            |
| Dupont wire      |       6         |             |               |       GND (Y)     |            |      -     |
| Dupont wire M/F  |       11        |             |               |                   |    27 E    |            |
| Dupont wire M/F  |       13        |             |               |                   |    36 H    |            |
| Dupont wire M/F  |       39        |             |               |                   |    40 I    |            |
| Dupont wire      |                 |     UP      |               |       OUT 1       |            |            |
| Dupont wire      |                 |    DOWN     |               |       OUT 2       |            |            |
| Dupont wire      |                 |             |      UP       |       OUT 4       |            |            |
| Dupont wire      |                 |             |     DOWN      |       OUT 3       |            |            |
| Dupont wire      |                 |             |               |       12V         |            |      +     |
| Dupont wire M/M  |                 |             |               |                   | 35 E - 40 F|            |
| Resistance       |                 |             |               |                   | 36 D - 36 F|            |
| Resistance       |                 |             |               |                   | 40 J - 36 J|            |
| HC-SR04          |                 |             |               |                   | 32A <-> 35A|            |

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
