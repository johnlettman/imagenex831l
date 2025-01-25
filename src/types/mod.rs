mod acceleration;
mod angle;
mod command;
mod config;
mod data_bits;
mod data_size_index;
mod direction;
mod file_header;
mod head_position;
mod logf;
mod mode;
mod motion_config;
mod motor_calibrate;
pub mod primitive;
mod profile_grid;
mod profile_point_detection;
mod range_code;
pub(crate) mod range_table;
mod sensor_available;
mod sensor_information;
mod sonar_return_header;
mod sonar_return_magic;
mod sonar_return_status;
mod sonar_type;
mod step_direction;
mod step_size;
mod transducer;
mod util;
mod zero;

pub use acceleration::Acceleration;
pub use angle::Angle;
pub use command::Command;
pub use config::Config;
pub use data_bits::DataBits;
pub use data_size_index::DataPoints;
pub use direction::Direction;
pub use file_header::FileHeader;
pub use head_position::HeadPosition;
pub use logf::Logf;
pub use mode::Mode;
pub use motion_config::MotionConfig;
pub use motor_calibrate::MotorCalibrate;
pub use profile_grid::ProfileGrid;
pub use profile_point_detection::ProfilePointDetection;
pub use range_code::RangeCode;
pub use range_table::RangeTable;
pub use sensor_available::SensorAvailable;
pub use sensor_information::SensorInformation;
pub use sonar_return_header::SonarReturnHeader;
pub use sonar_return_magic::SonarReturnMagic;
pub use sonar_return_status::SonarReturnStatus;
pub use sonar_type::SonarType;
pub use step_direction::StepDirection;
pub use step_size::StepSize;
pub use transducer::Transducer;
pub use zero::Zero;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "pyo3")]
#[pymodule]
pub(crate) fn types(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Acceleration>()?;
    module.add_class::<Angle>()?;
    module.add_class::<Command>()?;
    module.add_class::<Config>()?;
    module.add_class::<DataBits>()?;
    module.add_class::<DataPoints>()?;
    module.add_class::<Direction>()?;
    module.add_class::<FileHeader>()?;
    module.add_class::<HeadPosition>()?;
    module.add_class::<Logf>()?;
    module.add_class::<Mode>()?;
    module.add_class::<MotionConfig>()?;
    module.add_class::<MotorCalibrate>()?;
    module.add_class::<ProfileGrid>()?;
    module.add_class::<ProfilePointDetection>()?;
    module.add_class::<RangeCode>()?;
    module.add_class::<SensorAvailable>()?;
    module.add_class::<SensorInformation>()?;
    module.add_class::<SonarReturnHeader>()?;
    module.add_class::<SonarReturnMagic>()?;
    module.add_class::<SonarReturnStatus>()?;
    module.add_class::<SonarType>()?;
    module.add_class::<StepDirection>()?;
    module.add_class::<StepSize>()?;
    module.add_class::<Transducer>()?;
    module.add_class::<Zero>()?;
    Ok(())
}
