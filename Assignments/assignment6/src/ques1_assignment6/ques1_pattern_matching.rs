#[derive(PartialEq, Eq, Debug)]
/// enum as Coordinates having variants Abscissa and Ordinated.
pub enum Coordinates {
    Abscissa(i32),
    Ordinate(i32),
}
#[derive(PartialEq, Eq, Debug)]
/// Axis as enum having all quadrant and all other axis.
pub enum Axis {
    FirstQuadrant(Coordinates, Coordinates),
    SecondQuadrant(Coordinates, Coordinates),
    ThirdQuadrant(Coordinates, Coordinates),
    FourthQuadrant(Coordinates, Coordinates),
    XAxis(Coordinates, Coordinates),
    YAxis(Coordinates, Coordinates),
    Origin(Coordinates, Coordinates),
    None,
}

/// Function find_coordinates is used here to match points on axis of graph.
///
/// #Arguments
///
/// take_points as a tuple having i32 type variants to match.
///
/// #Return
///
/// returning Axis as output as position on graph.
pub fn find_coordinates(takes_points: (i32, i32)) -> Axis {
    match takes_points {
        (input_x, input_y) if input_x > 0 && input_y > 0 => Axis::FirstQuadrant(
            Coordinates::Abscissa(takes_points.0),
            Coordinates::Ordinate(takes_points.1),
        ),

        (input_x, input_y) if input_x < 0 && input_y > 0 => Axis::SecondQuadrant(
            Coordinates::Abscissa(takes_points.0),
            Coordinates::Ordinate(takes_points.1),
        ),

        (input_x, input_y) if input_x < 0 && input_y < 0 => Axis::ThirdQuadrant(
            Coordinates::Abscissa(takes_points.0),
            Coordinates::Ordinate(takes_points.1),
        ),

        (input_x, input_y) if input_x > 0 && input_y < 0 => Axis::FourthQuadrant(
            Coordinates::Abscissa(takes_points.0),
            Coordinates::Ordinate(takes_points.1),
        ),
        (input_x, input_y) if input_x != 0 && input_y == 0 => Axis::XAxis(
            Coordinates::Abscissa(takes_points.0),
            Coordinates::Ordinate(takes_points.1),
        ),
        (input_x, input_y) if input_x == 0 && input_y != 0 => Axis::YAxis(
            Coordinates::Abscissa(takes_points.0),
            Coordinates::Ordinate(takes_points.1),
        ),
        (input_x, input_y) if input_x == 0 && input_y == 0 => Axis::Origin(
            Coordinates::Abscissa(takes_points.0),
            Coordinates::Ordinate(takes_points.1),
        ),

        _ => Axis::None,
    }
}
