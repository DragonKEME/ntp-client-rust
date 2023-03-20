
use crate::ntp::Ntp;
pub mod ntp;

mod temps;

fn main() {
    let (request,delta, theta) = Ntp::ntp_request("1.fr.pool.ntp.org:123");
    println!("Local time start  : {}",temps::format_current_time());
    println!("server time       : {}",request.ntp_format(delta));
    println!("NetTime (delta)   : {}",delta);
    println!("Difference (theta): {}",theta);
    println!("corrected time end: {}",temps::format_corrected_current_time(theta));
}