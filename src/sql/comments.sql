COMMENT ON SCHEMA convert IS 'Contains functions created by convert extension. Helpful conversion functions for speed, distance, area, power, and travel time.';

COMMENT ON FUNCTION convert.dist_mi_to_km IS 'Distance conversion: miles to kilometers';
COMMENT ON FUNCTION convert.dist_km_to_mi IS 'Distance conversion: kilometers to miles';
COMMENT ON FUNCTION convert.dist_ft_to_m IS 'Distance conversion: feet to meters';
COMMENT ON FUNCTION convert.dist_m_to_ft IS 'Distance conversion: meters to feet';
COMMENT ON FUNCTION convert.dist_km_to_m IS 'Distance conversion: kilometers to meters';
COMMENT ON FUNCTION convert.dist_ft_to_mi IS 'Distance conversion: feet to miles';
COMMENT ON FUNCTION convert.dist_mi_to_ft IS 'Distance conversion: miles to feet';
COMMENT ON FUNCTION convert.dist_m_to_mi IS 'Distance conversion: meters to miles';
COMMENT ON FUNCTION convert.dist_m_to_km IS 'Distance conversion: meters to kilometers';
COMMENT ON FUNCTION convert.speed_mph_to_kmhr IS 'Speed conversion: miles per hour to kilometers per hour';
COMMENT ON FUNCTION convert.speed_kmhr_to_mph IS 'Speed conversion: kilometers per hour to miles per hour';
COMMENT ON FUNCTION convert.speed_kmhr_to_m_s IS 'Speed conversion: kilometers per hour to meters per second';
COMMENT ON FUNCTION convert.speed_mph_to_m_s IS 'Speed conversion: miles per hour to meters per second';
COMMENT ON FUNCTION convert.speed_m_s_to_mph IS 'Speed conversion: meters per second to miles per hour';
COMMENT ON FUNCTION convert.speed_m_s_to_kmhr IS 'Speed conversion: meters per second to kilometers per hour';
COMMENT ON FUNCTION convert.ttt_meters_m_s IS 'Time To Travel: Input uses meters and meters-per-second (m_s). Returns seconds.';
COMMENT ON FUNCTION convert.ttt_meters_km_hr_to_seconds IS 'Time To Travel: Input uses meters and kilometers per hour. Returns seconds.';

COMMENT ON FUNCTION convert.power_dbm_to_watts IS 'Power conversion: dBm to Watts';
COMMENT ON FUNCTION convert.power_watts_to_dbm IS 'Power conversion: Watts to dBm';

COMMENT ON FUNCTION convert.area_m2_to_km2 IS 'Area conversion: square meters to square kilometers';
COMMENT ON FUNCTION convert.area_m2_to_ft2 IS 'Area conversion: square meters to square feet';
COMMENT ON FUNCTION convert.area_ft2_to_m2 IS 'Area conversion: square feet to square meters';
COMMENT ON FUNCTION convert.area_ft2_to_mi2 IS 'Area conversion: square feet to square miles';
COMMENT ON FUNCTION convert.area_mi2_to_ft2 IS 'Area conversion: square miles to square feet';
COMMENT ON FUNCTION convert.area_mi2_to_acre IS 'Area conversion: square miles to acres';
COMMENT ON FUNCTION convert.area_acre_to_mi2 IS 'Area conversion: acres to square miles';
COMMENT ON FUNCTION convert.area_acre_to_km2 IS 'Area conversion: acres to square kilometers';

