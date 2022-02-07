const EARTH_RADIUS_METRES:f64		= 6.3781e+6_f64;
const EARTH_METRES_PER_RADIAN:f64	= EARTH_RADIUS_METRES;
const EARTH_METRES_PER_DEGREE:f64	= EARTH_METRES_PER_RADIAN * std::f64::consts::PI / 180.0;

// Yes indeed! The Earth is an `oblate spheroid`, but the difference is very small!

pub fn convert_metres_to_degrees(metres: f64) -> f64{
	metres / EARTH_METRES_PER_DEGREE
}

pub fn convert_degrees_to_metres(degrees: f64) -> f64{
	degrees * EARTH_METRES_PER_DEGREE
}