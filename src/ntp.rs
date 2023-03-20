use std::net::{ToSocketAddrs, UdpSocket};

use chrono::{Duration, NaiveDateTime, Utc};

const NTP_TIMESTAMP_DELTA: i64 = 2208988800;
const NTP_SIZE: usize = 48;

#[derive(Debug)]
pub struct Ntp {
    leap_version_mode: u8,
    stratum: u8,
    poll: u8,
    precision: u8,
    root_delay: u32,
    root_dispersion: u32,
    reference_id: u32,

    // Reference timestamp (s: seconds, f: float part)
    reference_t_s: u32,
    reference_t_f: u32,

    // Origin timestamp (s: seconds, f: float part)
    origin_t_s: u32,
    origin_t_f: u32,

    // Receive timestamp (s: seconds, f: float part)
    receive_t_s: u32,
    receive_t_f: u32,

    // Transmit timestamp (s: seconds, f: float part)
    transmit_t_s: u32,
    transmit_t_f: u32,

}

impl Ntp {
    fn init_client(version: u8) -> Ntp {
        Ntp {
            leap_version_mode: Ntp::leap_version_mode(0, version, 3),

            // Stratum undefined (client)
            stratum: 0,

            // Default : 10 (1024 seconds)
            poll: 10,

            // Default : 32
            precision: 32,

            root_delay: 0,
            root_dispersion: 0,
            reference_id: 0,

            // Reference timestamp (s: seconds, f: float part)
            reference_t_s: 0,
            reference_t_f: 0,

            // Origin timestamp (s: seconds, f: float part)
            origin_t_s: 0,
            origin_t_f: 0,

            // Receive timestamp (s: seconds, f: float part)
            receive_t_s: 0,
            receive_t_f: 0,

            // Transmit timestamp (s: seconds, f: float part)
            transmit_t_s: 0,
            transmit_t_f: 0,
        }
    }
    fn leap_version_mode(leap: u8, version: u8, mode: u8) -> u8 {
        let mut lvm = leap << 6;
        lvm |= version << 3;
        lvm |= mode;
        lvm
    }

    pub fn ntp_format(&self, delta: Duration) -> String {
        let time = self.transmit_datetime() + (delta / 2);
        time.format("[%Y-%m-%d|%H:%M:%S,%f]").to_string()
    }

    pub fn reference_datetime(&self) -> NaiveDateTime {
        // Remove NTP-DELTA and convert u32 to i64 for naiveDateTime
        let second = i64::from(self.reference_t_s) - NTP_TIMESTAMP_DELTA;
        // Convert ntp fraction to nanosecond (ntp fraction ~= 2.3*10^-9)
        let nanosecond = self.reference_t_f / 23;

        NaiveDateTime::from_timestamp_opt(second, nanosecond).unwrap()
    }

    pub fn origin_datetime(&self) -> NaiveDateTime {
        // Remove NTP-DELTA and convert u32 to i64 for naiveDateTime
        let second = i64::from(self.origin_t_s) - NTP_TIMESTAMP_DELTA;
        // Convert ntp fraction to nanosecond (ntp fraction ~= 2.3*10^-9)
        let nanosecond = self.origin_t_f / 23;

        NaiveDateTime::from_timestamp_opt(second, nanosecond).unwrap()
    }

    pub fn receive_datetime(&self) -> NaiveDateTime {
        // Remove NTP-DELTA and convert u32 to i64 for naiveDateTime
        let second = i64::from(self.receive_t_s) - NTP_TIMESTAMP_DELTA;
        // Convert ntp fraction to nanosecond (ntp fraction ~= 2.3*10^-9)
        let nanosecond = self.receive_t_f / 23;

        NaiveDateTime::from_timestamp_opt(second, nanosecond).unwrap()
    }

    pub fn transmit_datetime(&self) -> NaiveDateTime {
        // Remove NTP-DELTA and convert u32 to i64 for naiveDateTime
        let second = i64::from(self.transmit_t_s) - NTP_TIMESTAMP_DELTA;
        // Convert ntp fraction to nanosecond (ntp fraction ~= 2.3*10^-9)
        let nanosecond = self.transmit_t_f / 23;

        NaiveDateTime::from_timestamp_opt(second, nanosecond).unwrap()
    }

    pub fn from_slice(slice: [u8; NTP_SIZE]) -> Ntp {
        Ntp {
            leap_version_mode: u8::from_be(slice[0]),
            stratum: u8::from_be(slice[1]),
            poll: u8::from_be(slice[2]),
            precision: u8::from_be(slice[3]),

            root_delay: u32::from_be_bytes(<[u8; 4]>::try_from(&slice[4..8]).unwrap()),
            root_dispersion: u32::from_be_bytes(<[u8; 4]>::try_from(&slice[8..12]).unwrap()),
            reference_id: u32::from_be_bytes(<[u8; 4]>::try_from(&slice[12..16]).unwrap()),

            reference_t_s: u32::from_be_bytes(<[u8; 4]>::try_from(&slice[16..20]).unwrap()),
            reference_t_f: u32::from_be_bytes(<[u8; 4]>::try_from(&slice[20..24]).unwrap()),

            origin_t_s: u32::from_be_bytes(<[u8; 4]>::try_from(&slice[24..28]).unwrap()),
            origin_t_f: u32::from_be_bytes(<[u8; 4]>::try_from(&slice[28..32]).unwrap()),

            receive_t_s: u32::from_be_bytes(<[u8; 4]>::try_from(&slice[32..36]).unwrap()),
            receive_t_f: u32::from_be_bytes(<[u8; 4]>::try_from(&slice[36..40]).unwrap()),

            transmit_t_s: u32::from_be_bytes(<[u8; 4]>::try_from(&slice[40..44]).unwrap()),
            transmit_t_f: u32::from_be_bytes(<[u8; 4]>::try_from(&slice[44..48]).unwrap()),
        }
    }

    pub fn to_slice(&self) -> [u8; NTP_SIZE] {
        let mut tab = vec![
            self.leap_version_mode,
            self.stratum,
            self.poll,
            self.precision,
        ];

        tab.extend(self.root_delay.to_be_bytes());
        tab.extend(self.root_dispersion.to_be_bytes());
        tab.extend(self.reference_id.to_be_bytes());

        tab.extend(self.reference_t_s.to_be_bytes());
        tab.extend(self.reference_t_f.to_be_bytes());

        tab.extend(self.origin_t_s.to_be_bytes());
        tab.extend(self.origin_t_f.to_be_bytes());

        tab.extend(self.receive_t_s.to_be_bytes());
        tab.extend(self.receive_t_f.to_be_bytes());

        tab.extend(self.transmit_t_s.to_be_bytes());
        tab.extend(self.transmit_t_f.to_be_bytes());

        <[u8; NTP_SIZE]>::try_from(tab.as_slice()).unwrap()
    }
    pub fn ntp_request(server: &str) -> (Ntp, Duration, Duration) {
        // Init and make socket
        let ntp = Ntp::init_client(4);
        let server = server.to_socket_addrs().unwrap().next().unwrap();
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

        // Convert ntp to u8 array
        let bin_send = ntp.to_slice();
        // Note send time
        let time_send = Utc::now().naive_utc();

        socket.send_to(bin_send.as_slice(), server).expect("Err send socket");

        let mut bin_recv = [0u8; NTP_SIZE];
        socket.recv_from(&mut bin_recv).expect("error recv");

        // Note receive time
        let time_receive = Utc::now().naive_utc();

        let ntp = Ntp::from_slice(bin_recv);

        // Delta is the duration of the two transmission 1: client to server and 2: server to client
        let delta = (time_receive - time_send) - (ntp.receive_datetime() - ntp.transmit_datetime());

        // Theta is the difference between the local time and distant time
        let theta = ((ntp.receive_datetime() - time_send) / 2) + ((ntp.transmit_datetime() - time_receive) / 2);
        (ntp, delta, theta)
    }
}
