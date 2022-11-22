use pgx::prelude::*;

pgx::pg_module_magic!();



#[pg_extern]
fn dist_mi_to_ft(miles: f64) -> f64 {
    miles * 5280.0
}

#[pg_extern]
fn dist_ft_to_mi(feet: f64) -> f64 {
    feet / 5280.0
}


#[pg_extern]
fn dist_ft_to_m(feet: f64) -> f64 {
    feet * 0.3048
}

#[pg_extern]
fn dist_m_to_km(meters: f64) -> f64 {
    meters / 1000.0
}

#[pg_extern]
fn dist_km_to_m(kilometers: f64) -> f64 {
    kilometers * 1000.0
}


#[pg_extern]
fn dist_mi_to_km(miles: f64) -> f64 {
    dist_m_to_km(dist_ft_to_m(dist_mi_to_ft(miles)))
}


#[pg_extern]
fn dist_m_to_ft(meters: f64) -> f64 {
    meters * 3.28084
}

#[pg_extern]
fn dist_m_to_mi(meters: f64) -> f64 {
    dist_ft_to_mi(dist_m_to_ft(meters))
}

#[pg_extern]
fn dist_km_to_mi(kilometers: f64) -> f64 {
    dist_ft_to_mi(dist_m_to_ft(dist_km_to_m(kilometers)))
}


// mph and km/hr
#[pg_extern]
fn speed_mph_to_kmhr(miles_per_hour: f64) -> f64 {
    miles_per_hour * 1.609344
}

#[pg_extern]
fn speed_kmhr_to_mph(kilometers_per_hour: f64) -> f64 {
    kilometers_per_hour * 0.621
}


#[pg_extern]
fn speed_kmhr_to_m_s(kmhr: f64) -> f64 {
    kmhr * 0.27777777777778
}


// speed + distance -> time
#[pg_extern]
fn ttt_meters_m_s(length_meters: f64, meters_per_second: f64) -> f64 {
    length_meters / meters_per_second
}


extension_sql_file!("sql/comments.sql",
    finalize
);



#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::prelude::*;

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
