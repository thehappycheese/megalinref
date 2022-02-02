import math

EARTH_RADIUS_METRES		= 6.3781e+6
EARTH_METRES_PER_RADIAN	= EARTH_RADIUS_METRES
EARTH_METRES_PER_DEGREE	= EARTH_METRES_PER_RADIAN * math.pi / 180.0

def convert_metres_to_degrees(metres):
	return metres / EARTH_METRES_PER_DEGREE

def convert_degrees_to_metres(degrees):
	return degrees * EARTH_METRES_PER_DEGREE