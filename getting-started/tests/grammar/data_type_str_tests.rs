use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// str 类型测试

/// str 有一个 parse() 方法可以将字符串切片转换为其他类型
/// 内部借助 FromStr::from_str(self) 实现
#[test]
fn test_parse() {
    // 简单类型转换
    let four = "4".parse::<u32>().unwrap();
    assert_eq!(4u32, four);

    // 官方示例只展示了简单类型的转换，其实可以通过重写 from_str() 方法实现更复杂的转换，参考 ripgrep printer color.rs 中实现
    // 举个汽车的例子
    #[derive(Debug)]
    struct VehicleSpec {
        ty: VehicleType,
        value: VehicleValue,
    }

    #[derive(Debug)]
    enum VehicleType {
        Trunk,
        Car,
    }

    enum SpecType {
        Wheel,
        Seat,
    }

    #[derive(Debug)]
    enum VehicleValue {
        Wheel(u8),
        Seat(u8),
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum VehicleError {
        UnrecognizedVehicleType(String),
        UnrecognizedSpecType(String),
        InvalidFormat(String),
        // UnrecognizedSpecValue(String),
    }

    impl std::error::Error for VehicleError {}

    impl Display for VehicleError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match *self {
                VehicleError::UnrecognizedVehicleType(ref name) => write!(
                    f,
                    "unrecognized output type '{}'. Choose from: trunk、car.",
                    name,
                ),
                VehicleError::UnrecognizedSpecType(ref name) => write!(
                    f,
                    "unrecognized spec type '{}'. Choose from: wheel、seat.",
                    name,
                ),
                VehicleError::InvalidFormat(ref original) => write!(
                    f,
                    "invalid vehicle spec format: '{}'. Valid format \
                 is '(trunk|car):(wheel|seat):(value)'.",
                    original,
                ),
            }
        }
    }

    // impl VehicleError {
    //     fn from_parse_error(err: std::result::Result<T, E>) -> VehicleError {
    //         VehicleError::UnrecognizedSpecValue(
    //             err
    //         )
    //     }
    // }

    impl FromStr for VehicleSpec {
        type Err = VehicleError;

        fn from_str(s: &str) -> Result<Self, VehicleError> {
            let pieces: Vec<&str> = s.split(':').collect();
            if pieces.len() <= 1 || pieces.len() > 3 {
                return Err(VehicleError::InvalidFormat(s.to_string()));
            }
            let vt: VehicleType = pieces[0].parse()?;
            match pieces[1].parse()? {
                SpecType::Wheel => {
                    let wheels: u8 = pieces[2].parse().unwrap();
                    Ok(VehicleSpec { ty: vt, value: VehicleValue::Wheel(wheels) })
                }
                SpecType::Seat => {
                    let seats: u8 = pieces[2].parse().unwrap();
                    Ok(VehicleSpec { ty: vt, value: VehicleValue::Seat(seats) })
                }
            }
        }
    }

    impl FromStr for VehicleType {
        type Err = VehicleError;

        fn from_str(s: &str) -> Result<VehicleType, VehicleError> {
            match &*s.to_lowercase() {
                "trunk" => Ok(VehicleType::Trunk),
                "car" => Ok(VehicleType::Car),
                _ => Err(VehicleError::UnrecognizedVehicleType(s.to_string())),
            }
        }
    }

    impl FromStr for SpecType {
        type Err = VehicleError;

        fn from_str(s: &str) -> Result<Self, VehicleError> {
            match &*s.to_lowercase() {
                "wheel" => Ok(SpecType::Wheel),
                "seat" => Ok(SpecType::Seat),
                _ => Err(VehicleError::UnrecognizedSpecType(s.to_string())),
            }
        }
    }

    let vehicle_vec: Vec<VehicleSpec> = vec![
        "trunk:wheel:6".parse().unwrap(),
        "trunk:seat:2".parse().unwrap(),
        "car:wheel:4".parse().unwrap(),
        "car:seat:4".parse().unwrap(),
    ];

    // [VehicleSpec { ty: Trunk, value: Wheel(6) }, VehicleSpec { ty: Trunk, value: Seat(2) }, VehicleSpec { ty: Car, value: Wheel(4) }, VehicleSpec { ty: Car, value: Seat(4) }]
    println!("{vehicle_vec:?}")
}