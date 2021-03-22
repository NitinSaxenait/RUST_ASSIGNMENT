/// Geometrics_Series a struct with 3 variants all of i32 types.
pub struct GeometricSeries {
    pub first_number: i32,
    pub current_number: i32,
    pub ratio: i32,
}
/// Iterator trait is used here having a method geometric_series which is returning a Vec<i32> type.
pub trait Iterator {
    fn geometric_series(&self) -> Vec<i32>;
}

/// Iterator is implemented for GeometricSeries struct type.
///
/// Function geometric_series is calculating the series using geometric progression formula.
///
/// based on  -> a,ar,ar^2,ar^3
///
/// #Arguments
///
/// -> &self is taking variants of struct Geometric_Series.
///
/// #Return
///
/// A Vec<i32> returns as geometric series pattern.

impl Iterator for GeometricSeries {
    fn geometric_series(&self) -> Vec<i32> {
        let mut collector: Vec<i32> = Vec::new();

        for index in 0..11 {
            collector.push(self.first_number * self.ratio.pow(index as u32));
        }
        collector
    }
}
