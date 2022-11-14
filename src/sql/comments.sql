COMMENT ON SCHEMA convert IS 'Contains functions created by convert extension. Helpful spatial/routing conversion functions.';

COMMENT ON FUNCTION convert.dist_mi_to_km IS 'Distance conversion - miles to kilometers';
COMMENT ON FUNCTION convert.dist_km_to_mi IS 'Distance conversion - kilometers to miles';
COMMENT ON FUNCTION convert.dist_ft_to_m IS 'Distance conversion - feet to meters';
COMMENT ON FUNCTION convert.dist_m_to_ft IS 'Distance conversion - meters to feet';
COMMENT ON FUNCTION convert.speed_mph_to_kmhr IS 'Speed conversion - miles per hour to kilometers per hour';
COMMENT ON FUNCTION convert.speed_kmhr_to_mph IS 'Speed conversion - kilometers per hour to miles per hour';
COMMENT ON FUNCTION convert.ttt_meters_m_s IS 'TTT = Time To Travel.  Input: Meters and Meters per Second (m_s). Returns Seconds.';


