mod base_data;
pub use base_data::*;

#[cfg(feature = "nalgebra")]
mod nalgebra_data;
#[cfg(feature = "nalgebra")]
#[allow(unused_imports)]
pub use nalgebra_data::*;

#[cfg(feature = "rulinalg")]
mod rulinalg_data;
#[cfg(feature = "rulinalg")]
#[allow(unused_imports)]
pub use rulinalg_data::*;

#[cfg(feature = "ndarray")]
mod ndarray_data;
#[cfg(feature = "ndarray")]
#[allow(unused_imports)]
pub use ndarray_data::*;

#[cfg(feature = "csv")]
mod csv_data;
#[cfg(feature = "csv")]
#[allow(unused_imports)]
pub use csv_data::*;

#[cfg(feature = "polars")]
mod polars_data;
#[cfg(feature = "polars")]
#[allow(unused_imports)]
pub use polars_data::*;
