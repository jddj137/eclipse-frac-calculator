use marlu::sexagesimal::{sexagesimal_dms_string_to_degrees, sexagesimal_hms_string_to_degrees};
use measurements::angle::Angle;
use measurements::length::Length;
use std::f64::consts::PI;
use std::time::Duration;

const EARTH_RADIUS_M: f64 = 6378.145e3f64; // from BMW, Appendix A
const EARTH_GRAVITATION_PARAM_M3_SEC2: f64 = 3.986012e5f64 * 1.0e9f64; // from BMW, Appendix A

fn calculate_beta_angle(
    right_ascension_sun: Angle,
    declination_sun: Angle,
    inclination_sat: Angle,
    right_ascension_ascending_node_sat: Angle,
) -> Angle {
    let sin_gamma = right_ascension_sun.as_radians().sin();
    let cos_gamma = right_ascension_sun.as_radians().cos();

    let sin_epsilon = declination_sun.as_radians().sin();
    let cos_epsilon = declination_sun.as_radians().cos();

    let sin_i = inclination_sat.as_radians().sin();
    let cos_i = inclination_sat.as_radians().cos();

    let sin_omega = right_ascension_ascending_node_sat.as_radians().sin();
    let cos_omega = right_ascension_ascending_node_sat.as_radians().cos();

    let a: f64 = cos_gamma * sin_omega * sin_i;
    let b: f64 = sin_gamma * cos_epsilon * cos_omega * sin_i;
    let c: f64 = sin_gamma * sin_epsilon * cos_i;

    Angle::from_radians((a - b + c).asin())
}

fn calculate_beta_angle_star(altitude: Length) -> Angle {
    let a: f64 = EARTH_RADIUS_M / (EARTH_RADIUS_M + altitude.as_meters());

    Angle::from_radians(a.asin())
}

fn calculate_eclipse_fraction(altitude: Length, beta_angle: Angle) -> f64 {
    let beta_angle_star: Angle = calculate_beta_angle_star(altitude);

    println!("beta_angle_star (deg): {}", beta_angle_star.as_degrees());

    println!(
        "beta_angle.as_radians().abs(): {}",
        beta_angle.as_degrees().abs()
    );
    println!(
        "beta_angle_star.as_radians().abs(): {}",
        beta_angle_star.as_degrees()
    );

    if beta_angle.as_radians().abs() >= beta_angle_star.as_radians() {
        return 0.0;
    }

    let h_m: f64 = altitude.as_meters();

    let numer: f64 = f64::sqrt(h_m * h_m + 2.0 * EARTH_RADIUS_M * h_m);
    let denom: f64 = (EARTH_RADIUS_M + h_m) * beta_angle.as_radians().cos();

    (numer / denom).acos()
}

fn calculate_orbit_period_seconds(
    semi_major_axis: Length,
    gravitation_parameter_m3_s2: f64,
) -> Duration {
    let orbit_period_s: f64 =
        f64::sqrt((semi_major_axis.as_meters()).powi(3) / gravitation_parameter_m3_s2) * 2.0 * PI;

    Duration::from_secs_f64(orbit_period_s)
}

fn calculate_eclipse_time_s(altitude: Length, beta_angle: Angle) -> f64 {
    let semi_major_axis = Length::from_meters(EARTH_RADIUS_M + altitude.as_meters());

    let period: Duration =
        calculate_orbit_period_seconds(semi_major_axis, EARTH_GRAVITATION_PARAM_M3_SEC2);
    let eclipse_fraction: f64 = calculate_eclipse_fraction(altitude, beta_angle);

    println!("period: {}", period.as_secs());
    println!("eclipse_fraction: {eclipse_fraction}");

    period.as_secs_f64() * eclipse_fraction
}

fn main() {
    println!("Running leo-eclipse-fraction-calculator...");

    let right_ascension_sun = sexagesimal_hms_string_to_degrees("23.0h11.0m31.47s").unwrap();
    let declination_sun = sexagesimal_dms_string_to_degrees("-05.0d12.0m00.8s").unwrap();

    println!("right_ascension_sun: {right_ascension_sun}");
    println!("declination_sun: {declination_sun}");

    let beta_angle: Angle = calculate_beta_angle(
        Angle::from_degrees(right_ascension_sun),
        Angle::from_degrees(declination_sun),
        Angle::from_degrees(30.0),
        Angle::from_degrees(0.0),
    );

    println!("beta_angle (deg): {}", beta_angle.as_degrees());

    let altitude = Length::from_kilometers(600.0);

    let eclipse_time = calculate_eclipse_time_s(altitude, beta_angle);
    println!("Eclipse Time (sec): {eclipse_time}");
    println!("Eclipse Time (min): {}", eclipse_time / 60.0);
}
