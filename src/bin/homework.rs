#![allow(unused_variables)]
fn main() {

    println!("Pluralsight Homework");

    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;
    
    let wuhan_latitude: f64 = 30.5928;
    let wuhan_longitude: f64= 114.3055;

    let beijing_latitude: f64 = 39.9042;
    let beijing_longitude: f64 = 116.4074;

    let trondheim_latitude: f64 = 63.4305;
    let trondheim_longitude: f64 = 10.3951;

    let wuhan_latitude_in_radians = wuhan_latitude.to_radians();
    let beijing_latitude_in_radians = beijing_latitude.to_radians();
    let trondheim_latitude_in_radians = trondheim_latitude.to_radians();

    let delta_latitude = (wuhan_latitude - trondheim_latitude).to_radians();
    let delta_longitude = (wuhan_longitude - trondheim_longitude).to_radians();

    let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2) 
    + wuhan_latitude_in_radians.cos() 
    * trondheim_latitude_in_radians.cos() * f64::powi((delta_longitude / 2.0).sin(), 2);

    let central_angle = 2.0 * inner_central_angle.sqrt().asin();

    let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;

    println!("Distance between Wuhan and Trondheim is {} km", distance);
}