<div align="center">
<br>

![IMAGENEX Model 831L Pipe Profiling Sonar][imagenex-831l-image]

# imagenex831l
An unofficial Rust library for interacting with data from
[IMAGENEX 831L Pipe Profiling Sonars][imagenex-831l].

[![License][shield-license]][url-license]
![Programming Language][shield-language]
[![Tests][shield-tests]][url-tests]
[![Coverage][shield-codecov]][url-codecov]

<hr>
</div>

## Features
- Efficient data serialization and deserialization in the 831L format
- File-based reading using memory maps
- Direct `struct` data representation without intermediary functions
- PLANNED: Python bindings using [pyo3](https://github.com/PyO3/pyo3)
- PLANNED: WebAssembly target with TypeScript support using [tsify](https://github.com/madonoharu/tsify)

## Example
```rust
use project_root::get_project_root;
use imagenex831l::File;

fn main() {
    let mut path = get_project_root().unwrap();
    path.push("sample");
    path.push("27JUL2023-101914.31l");

    let mut shot_file = File::open(&path).expect("Failed to open shot file for reading");
    for shot in &mut shot_file {
        println!("{shot:#?}");
        break;
    }    
}
```
<details>
<summary>Output</summary>

```
Shot {
    header: FileHeader {
        data_size_index: X250Bytes,
        total_length: 512,
        data_length: 283,
        datetime: 2023-07-27T10:19:19.690Z,
        sensor_available: NotPresent,
        motion: MotionConfig {
            direction: Clockwise,
            transducer: Up,
            mode: Polar,
            step_size: Fast,
        },
        start_gain: 6,
        sector_size: 360,
        train_angle: 360,
        range_code: X1m,
        absorption: 1.7,
        config: Config {
            profile_grid: Off,
            zero: Up,
            data_bits: X14Bits,
            logf: X20dB,
        },
        pulse_length: 100,
        sound_velocity: 1500.0,
        operating_frequency: 8,
        real_time_prf: 189.47,
        sensor_information: SensorInformation {
            pitch_valid: true,
            roll_valid: true,
            distance_valid: true,
        },
        pitch: 0.0,
        roll: 0.0,
        distance: 0.0,
    },
    sonar_return: SonarReturn {
        header: SonarReturnHeader {
            magic: IMX,
            sonar_type: Scanning,
            status: SonarReturnStatus {
                range_error: true,
                frequency_error: false,
                internal_sensor_error: false,
                calibration_error: false,
                switches_accepted: false,
            },
            head_position: HeadPosition {
                angle: 93.600006,
                direction: Clockwise,
            },
            range_index: X1m,
            profile_range: 0.02,
            data_length: 250,
            roll_angle: Angle {
                angle: -65.25,
                new_data: true,
                error_alarm: false,
            },
            pitch_angle: Angle {
                angle: 11.75,
                new_data: false,
                error_alarm: false,
            },
            roll_acceleration: Acceleration {
                acceleration: -907.95667,
                new_data: true,
                error_alarm: false,
            },
            pitch_acceleration: Acceleration {
                acceleration: 203.8569,
                new_data: true,
                error_alarm: false,
            },
        },
        data: [ /* DATA HERE */  ],
        termination_byte: 252,
    },
}
```

</details>

[imagenex-831l-image]: .github/assets/831L-transparent.png
[imagenex-831l]: https://imagenex.com/products/831l-pipe-profiling

[shield-license]: https://img.shields.io/github/license/johnlettman/imagenex831l?style=for-the-badge
[url-license]: ./LICENSE

[shield-language]: https://img.shields.io/github/languages/top/johnlettman/imagenex831l?style=for-the-badge

[shield-tests]: https://img.shields.io/github/actions/workflow/status/johnlettman/imagenex831l/ci.yaml?style=for-the-badge&label=tests
[url-tests]: https://github.com/johnlettman/imagenex831l/actions/workflows/ci.yaml

[shield-codecov]: https://img.shields.io/codecov/c/github/johnlettman/imagenex831l?style=for-the-badge
[url-codecov]: https://app.codecov.io/gh/johnlettman/imagenex831l
