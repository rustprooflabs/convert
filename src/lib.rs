use pgrx::prelude::*;

pgrx::pg_module_magic!();

/////////////////////////////
// Distance conversions
#[pg_extern(immutable)]
fn dist_mi_to_ft(miles: f64) -> f64 {
    miles * 5280.0
}

#[pg_extern(immutable)]
fn dist_ft_to_mi(feet: f64) -> f64 {
    feet / 5280.0
}


#[pg_extern(immutable)]
fn dist_ft_to_m(feet: f64) -> f64 {
    feet * 0.3048
}

#[pg_extern(immutable)]
fn dist_m_to_km(meters: f64) -> f64 {
    meters / 1000.0
}

#[pg_extern(immutable)]
fn dist_km_to_m(kilometers: f64) -> f64 {
    kilometers * 1000.0
}


#[pg_extern(immutable)]
fn dist_mi_to_km(miles: f64) -> f64 {
    dist_m_to_km(dist_ft_to_m(dist_mi_to_ft(miles)))
}


#[pg_extern(immutable)]
fn dist_m_to_ft(meters: f64) -> f64 {
    meters * 3.28084
}

#[pg_extern(immutable)]
fn dist_m_to_mi(meters: f64) -> f64 {
    dist_ft_to_mi(dist_m_to_ft(meters))
}

#[pg_extern(immutable)]
fn dist_km_to_mi(kilometers: f64) -> f64 {
    dist_ft_to_mi(dist_m_to_ft(dist_km_to_m(kilometers)))
}


/////////////////////////////
// Speed conversions
#[pg_extern(immutable)]
fn speed_mph_to_kmhr(miles_per_hour: f64) -> f64 {
    miles_per_hour * 1.609344
}

#[pg_extern(immutable)]
fn speed_kmhr_to_mph(kilometers_per_hour: f64) -> f64 {
    kilometers_per_hour * 0.621
}


#[pg_extern(immutable)]
fn speed_kmhr_to_m_s(kmhr: f64) -> f64 {
    kmhr * 0.27777777777778
}

#[pg_extern(immutable)]
fn speed_mph_to_m_s(miles_per_hour: f64) -> f64 {
    speed_kmhr_to_m_s(speed_mph_to_kmhr(miles_per_hour))
}

#[pg_extern(immutable)]
fn speed_m_s_to_kmhr(meters_per_second: f64) -> f64 {
    meters_per_second * 3.6
}

#[pg_extern(immutable)]
fn speed_m_s_to_mph(meters_per_second: f64) -> f64 {
    speed_kmhr_to_mph(speed_m_s_to_kmhr(meters_per_second))
}


/////////////////////////////
// Time-To-Travel conversions
#[pg_extern(immutable)]
fn ttt_meters_m_s(length_meters: f64, meters_per_second: f64) -> f64 {
    length_meters / meters_per_second
}


/////////////////////////////
// Power conversions
#[pg_extern(immutable)]
fn power_dbm_to_watts(dbm: f64) -> f64 {
    let base: f64 = 10.0;
    base.powf(dbm / 10.0) / 1000.0
}

#[pg_extern(immutable)]
fn power_watts_to_dbm(watts: f64) -> f64 {
    ( 10.0 * watts.log10() ) + 30.0
}


/////////////////////////////
// Area conversions
#[pg_extern(immutable)]
fn area_m2_to_km2(meters_squared: f64) -> f64 {
    meters_squared / 1000_f64.powf(2.0)
}

#[pg_extern(immutable)]
fn area_m2_to_ft2(meters_squared: f64) -> f64 {
    meters_squared * 10.76391
}

#[pg_extern(immutable)]
fn area_ft2_to_m2(feet_squared: f64) -> f64 {
    feet_squared * 0.09290304
}

#[pg_extern(immutable)]
fn area_ft2_to_mi2(feet_squared: f64) -> f64 {
    feet_squared / 27878400.0
}

#[pg_extern(immutable)]
fn area_mi2_to_ft2(square_miles: f64) -> f64 {
    square_miles * 27878400.0
}

#[pg_extern(immutable)]
fn area_mi2_to_acre(square_miles: f64) -> f64 {
    square_miles * 640.0
}

#[pg_extern(immutable)]
fn area_acre_to_mi2(acres: f64) -> f64 {
    acres / 640.0
}

#[pg_extern(immutable)]
fn area_acre_to_km2(acres: f64) -> f64 {
    area_m2_to_km2(area_ft2_to_m2(area_mi2_to_ft2(area_acre_to_mi2(acres))))
}



/////////////////////////////
// Add comments to functions
extension_sql_file!("sql/comments.sql",
    finalize
);


#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_dist_m_to_ft() {
        let expected = 3.6089240000000005;
        let actual = crate::dist_m_to_ft(1.1);
        let absolute_diff = (expected - actual).abs();
        assert!(absolute_diff <= f64::EPSILON);
    }

    #[pg_test]
    fn test_dist_mi_to_km() {
        let expected = 1.7702784000000003;
        let actual = crate::dist_mi_to_km(1.1);
        let absolute_diff = (expected - actual).abs();
        assert!(absolute_diff <= f64::EPSILON);
    }

    #[pg_test]
    fn test_dist_km_to_mi() {
        let expected = 2.7340333333333335;
        let actual = crate::dist_km_to_mi(4.4);
        let absolute_diff = (expected - actual).abs();
        assert!(absolute_diff <= f64::EPSILON);
    }

    #[pg_test]
    fn test_power_dbm_to_watts() {
        let expected = 1.0;
        let actual = crate::power_dbm_to_watts(30.0);
        let absolute_diff = (expected - actual).abs();
        assert!(absolute_diff <= f64::EPSILON);
    }

    #[pg_test]
    fn test_power_watts_to_dbm() {
        let expected = 30.0;
        let actual = crate::power_watts_to_dbm(1.0);
        let absolute_diff = (expected - actual).abs();
        assert!(absolute_diff <= f64::EPSILON);
    }

    #[pg_test]
    fn test_speed_mph_to_kmhr() {
        let expected = 1.609344;
        let actual = crate::speed_mph_to_kmhr(1.0);
        let absolute_diff = (expected - actual).abs();
        assert!(absolute_diff <= f64::EPSILON);
    }
    
    #[pg_test]
    fn test_speed_mph_to_m_s() {
        let expected = 0.4470400000000036;
        let actual = crate::speed_mph_to_m_s(1.0);
        let absolute_diff = (expected - actual).abs();
        assert!(absolute_diff <= f64::EPSILON, "Actual found: {actual}");
    }
    
    #[pg_test]
    fn test_speed_m_s_to_kmhr() {
        let expected = 15.120000000000001;
        let actual = crate::speed_m_s_to_kmhr(4.2);
        let absolute_diff = (expected - actual).abs();
        assert!(absolute_diff <= f64::EPSILON, "Actual found: {actual}");
    }

    #[pg_test]
    fn test_speed_m_s_to_mph() {
        let expected = 9.389520000000001;
        let actual = crate::speed_m_s_to_mph(4.2);
        let absolute_diff = (expected - actual).abs();
        assert!(absolute_diff <= f64::EPSILON, "Actual found: {actual}");
    }

    #[pg_test]
    fn test_area_m2_to_km2() {
        let expected = 402906.639836;
        let actual = crate::area_m2_to_km2(402906639836.0);
        let absolute_diff = (expected - actual).abs();
        assert!(absolute_diff <= f64::EPSILON, "Actual found: {actual}");
    }

    #[pg_test]
    fn test_area_acre_to_km2() {
        let expected = 0.1699679697408;
        let actual = crate::area_acre_to_km2(42.0);
        let absolute_diff = (expected - actual).abs();
        assert!(absolute_diff <= f64::EPSILON, "Actual found: {actual}");
    }

}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
