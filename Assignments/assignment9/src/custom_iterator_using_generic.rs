pub struct GeometricSeries {
    pub first_number: i32,
    pub current_number: i32,
    pub ratio: i32,
}
//
// iterator trait is used here:
//
// trait -> #Method
//
// geometric_series method is used which is returning a Vec<i32> type.
pub trait Iterator {
    fn geometric_series(&self) -> Vec<i32>;
}
//
// iterator is implemented for GeometricSeries struct type.
//
// function geometric_series is calculating the series using geometric progression formula.
//
// -> a,ar,ar^2,ar^3
//
// where first_number is a
// and current number is total elements.
// ratio is current_number/first_number
//
// #Arguments
//
// -> &self is used as a parameter for method.
//
// #Return
//
// A Vec<i32> is returning as collector containing series pattern.

impl Iterator for GeometricSeries {
    fn geometric_series(&self) -> Vec<i32> {
        let mut collector: Vec<i32> = Vec::new();

        for index in 0..11 {
            collector.push(self.first_number * self.ratio.pow(index as u32));
        }
        collector
    }
}
