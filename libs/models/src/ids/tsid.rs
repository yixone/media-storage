use chrono::Utc;

id_type! {
    /// 64-bit numeric identifier based on
    /// `timestamp (48 bits)` + `random sequence (16 bits)`
    #[derive(Copy)]
    TSID as i64
}

const TSID_EPOCH: u64 = 1735678800000;

impl TSID {
    /// Generates a new [`TSID`] based on the current timestamp
    ///
    /// Panics if the system time is set earlier than [`TSID_EPOCH`]
    pub fn new() -> Self {
        let now = Utc::now().timestamp_millis() as u64;

        if now < TSID_EPOCH {
            panic!("System clock before TSID_EPOCH");
        }

        let timestamp = now - TSID_EPOCH;
        let rand = rand::random::<u16>() as u64;

        // [ TIMESTAMP (48) ] | [ RANDOM (16) ]
        let inner = ((timestamp << 16) | (rand & 0xFFFF)) as i64;
        TSID(inner)
    }

    /// Returns the timestamp of the ID creation time
    pub fn timestamp(&self) -> i64 {
        (self.0 >> 16).saturating_add(TSID_EPOCH as i64)
    }
}
