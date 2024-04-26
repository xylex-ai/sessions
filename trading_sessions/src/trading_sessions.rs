use polars::prelude::*;

const SECONDS_PER_DAY: u32 = 86_400;
const SECONDS_PER_HOUR: u32 = 3_600;

use crate::{IdentifyTradingSession, SessionVerification, SessionColumn};


impl IdentifyTradingSession {
    /// Creates a new IdentifyTradingSession instance with the given Unix timestamp.
    ///
    /// # Arguments
    ///
    /// * `unix_timestamp` - A 32-bit unsigned integer representing the Unix timestamp in seconds.
    ///
    /// # Returns
    ///
    /// A new IdentifyTradingSession instance.
    pub fn new(unix_timestamp: u32) -> Self {
        Self { unix_timestamp }
    }

    /// Identifies the trading session based on the stored Unix timestamp in seconds.
    ///
    /// Returns a string representing the trading session based on the hour of the day in UTC.
    ///
    /// # Examples
    ///
    /// ```
    /// use trading_sessions::IdentifyTradingSession;
    ///
    /// let session_identifier = IdentifyTradingSession::new(1708574400);
    /// assert_eq!(session_identifier.identify_trading_session(), "Tokyo");
    /// ```
    pub fn identify_trading_session(&self) -> String {
        let utc_hour = (self.unix_timestamp % SECONDS_PER_DAY) / SECONDS_PER_HOUR;

        match utc_hour {
            0..=6 => "Tokyo".to_string(),            // 12:00 AM - 7:00 AM
            7..=8 => "Tokyo_London".to_string(),     // 7:00 AM - 9:00 AM
            9..=12 => "London".to_string(),          // 9:00 AM - 1:00 PM
            13..=15 => "London_NewYork".to_string(), // 1:00 PM - 4:00 PM
            16..=21 => "NewYork".to_string(),        // 4:00 PM - 10:00 PM
            _ => "Undefined".to_string(),
        }
    }
}



impl SessionVerification {
    /// Creates a new SessionVerification instance with the given Unix timestamp and session name.
    ///
    /// # Arguments
    ///
    /// * `unix_timestamp` - A 32-bit unsigned integer representing the Unix timestamp in seconds.
    /// * `session` - A string representing the trading session name.
    ///
    /// # Returns
    ///
    /// A new SessionVerification instance.
    pub fn new(unix_timestamp: u32, session: String) -> Self {
        Self { unix_timestamp, session }
    }
    /// Verifies if the given session string matches the trading session identified by the Unix timestamp.
    ///
    /// # Returns
    ///
    /// Returns true if the identified session matches the input session name; otherwise, returns false.
    pub fn verify(&self) -> bool {
        let session_identifier = IdentifyTradingSession::new(self.unix_timestamp);
        let identified_session = session_identifier.identify_trading_session();
        self.session == identified_session
    }
}



impl SessionColumn {
    pub fn new(lazyframe: LazyFrame) -> Self {
        Self { lazyframe }
    }

    /// Applies the trading session column transformation to the LazyFrame.
    pub fn apply_session_column(&mut self) {
        self.lazyframe = self.lazyframe.clone().with_column(
            when(((col("time") % lit(SECONDS_PER_DAY)) / lit(SECONDS_PER_HOUR))
                .lt_eq(lit(6))).then(lit("Tokyo"))
                .when(((col("time") % lit(SECONDS_PER_DAY)) / lit(SECONDS_PER_HOUR))
                .lt_eq(lit(8))).then(lit("Tokyo_London"))
                .when(((col("time") % lit(SECONDS_PER_DAY)) / lit(SECONDS_PER_HOUR))
                .lt_eq(lit(12))).then(lit("London"))
                .when(((col("time") % lit(SECONDS_PER_DAY)) / lit(SECONDS_PER_HOUR))
                .lt_eq(lit(15))).then(lit("London_NewYork"))
                .when(((col("time") % lit(SECONDS_PER_DAY)) / lit(SECONDS_PER_HOUR))
                .lt_eq(lit(21))).then(lit("NewYork"))
                .otherwise(lit("hello"))
                .alias("Session"));
    }
}