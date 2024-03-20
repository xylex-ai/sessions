use polars::prelude::*;

const SECONDS_PER_DAY: u32 = 86_400;
const SECONDS_PER_HOUR: u32 = 3_600;

/// Identifies the trading session based on the stored Unix timestamp in seconds.
///
/// Calculations are based on UK and USA normal time, i.e., NOT daylight saving time.
///
/// The trading session is determined by the hour of the day in UTC:
/// - Tokyo: 12:00 AM - 7:00 AM
/// - Tokyo_London: 7:00 AM - 9:00 AM
/// - London: 9:00 AM - 1:00 PM
/// - London_NewYork: 1:00 PM - 4:00 PM
/// - NewYork: 4:00 PM - 10:00 PM
/// - Undefined: Any other time
///
/// # Examples
///
/// ```
/// use trading_sessions::IdentifyTradingSession;
///
/// let session_identifier = IdentifyTradingSession::new(1708574400); // This timestamp corresponds to a time within the Tokyo session
/// assert_eq!(session_identifier.identify_trading_session(), "Tokyo");
///
/// let session_identifier = IdentifyTradingSession::new(1708596000); // This timestamp corresponds to a time within the London session
/// assert_eq!(session_identifier.identify_trading_session(), "London");
///
/// let session_identifier = IdentifyTradingSession::new(1708696800); // This timestamp corresponds to a time within the London_NewYork session
/// assert_eq!(session_identifier.identify_trading_session(), "London_NewYork");
/// ```
pub struct IdentifyTradingSession {
    pub unix_timestamp: u32,
}

impl IdentifyTradingSession {
    pub fn new(unix_timestamp: u32) -> Self {
        Self { unix_timestamp }
    }

    pub fn identify_trading_session(&self) -> String {
        let utc_hour = (self.unix_timestamp % SECONDS_PER_DAY) / SECONDS_PER_HOUR;

        match utc_hour {
            0..=6 => "Tokyo".to_string(),            // 12:00 AM - 7:00 AM
            7..=8 => "Tokyo_London".to_string(),     // 7:00 AM - 9:00 AM
            9..=12 => "London".to_string(),          // 9:00 AM - 1:00 PaM
            13..=15 => "London_NewYork".to_string(), // 1:00 PM - 4:00 PM
            16..=21 => "NewYork".to_string(),        // 4:00 PM - 10:00 PM
            _ => "Undefined".to_string(),
        }
    }
}

/// Verifies if the given session string matches the trading session identified by the Unix timestamp.
///
/// This struct takes a Unix timestamp and a session name as input. It uses the `IdentifyTradingSession` struct
/// to determine the trading session based on the timestamp. If the identified session matches the input session name,
/// it returns true; otherwise, it returns false.
///
/// # Examples
///
/// ```
/// use trading_sessions::SessionVerification;
///
/// // Assuming the timestamp corresponds to a time within the Tokyo session
/// let verifier = SessionVerification::new(1708574400, "Tokyo".to_string());
/// assert!(verifier.verify());
///
/// // Assuming the timestamp corresponds to a time within the London session but the session name is incorrect
/// let verifier = SessionVerification::new(1708596000, "Tokyo".to_string());
/// assert!(!verifier.verify());
///
/// // Assuming the timestamp corresponds to a time within the London_NewYork session
/// let verifier = SessionVerification::new(1708696800, "London_NewYork".to_string());
/// assert!(verifier.verify());
/// ```
pub struct SessionVerification {
    pub unix_timestamp: u32,
    pub session: String,
}

impl SessionVerification {
    pub fn new(unix_timestamp: u32, session: String) -> Self {
        Self { unix_timestamp, session }
    }

    pub fn verify(&self) -> bool {
        let session_identifier = IdentifyTradingSession::new(self.unix_timestamp);
        let identified_session = session_identifier.identify_trading_session();
        self.session == identified_session
    }
}

/// Adds a "Session" column to a `LazyFrame` based on Unix timestamps in a "time" column.
///
/// This method transforms the input `LazyFrame` by adding a new column named "Session".
/// The session is determined by the hour extracted from the Unix timestamp in the "time" column.
/// The mapping of hours to session names is as follows:
/// - Tokyo: 12:00 AM - 7:00 AM
/// - Tokyo_London: 7:00 AM - 9:00 AM
/// - London: 9:00 AM - 1:00 PM
/// - London_NewYork: 1:00 PM - 4:00 PM
/// - NewYork: 4:00 PM - 10:00 PM
/// - Any other time is labeled as "Undefined"
///
/// # Examples
///
/// ```
/// use polars::prelude::*;
/// use trading_sessions::SessionColumn;
///
/// // Create a DataFrame with 3 rows and one column named "time" containing Unix timestamps
/// let df = df! {
///     "time" => [1708574400, 1708596000, 1708696800]
/// }.unwrap();
///
/// // Convert DataFrame to LazyFrame
/// let lazy_frame = df.lazy();
///
/// // Create a SessionColumn instance and apply the session column transformation
/// let mut session_column = SessionColumn::new(lazy_frame);
/// session_column.apply_session_column();
///
/// // Collect the transformed LazyFrame back into a DataFrame for verification
/// let result_df = session_column.lazyframe.collect().unwrap();
///
/// // Verify that the "Session" column exists and contains the correct session names for each timestamp
///assert_eq!(result_df.column("Session").unwrap().str_value(0).unwrap(), "Tokyo");
///assert_eq!(result_df.column("Session").unwrap().str_value(1).unwrap(), "London");
///assert_eq!(result_df.column("Session").unwrap().str_value(2).unwrap(), "London_NewYork");
/// ```
///
/// Note: This example assumes the existence of a `sessions` module where `SessionColumn` is defined.
pub struct SessionColumn {
   pub lazyframe: LazyFrame,
}

impl SessionColumn {
    pub fn new(lazyframe: LazyFrame) -> Self {
        Self { lazyframe }
    }

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