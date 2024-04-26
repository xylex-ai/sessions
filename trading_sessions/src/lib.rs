//! # Trading Sessions
//!
//! A Rust crate for identifying and verifying trading sessions based on Unix timestamps.
//!
//! ## Getting Started
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! trading_sessions = "0.1.0"
//! ```
//!
//! ## Features
//!
//! - [`IdentifyTradingSession`](./struct.IdentifyTradingSession.html): Determine the trading session from a Unix timestamp.
//! - [`SessionVerification`](./struct.SessionVerification.html): Verify if a given session string matches the identified trading session.
//! - [`SessionColumn`](./struct.SessionColumn.html): Add a "Session" column to a `LazyFrame` based on Unix timestamps.
//!
//! ## Examples
//!
//! ### IdentifyTradingSession
//!
//! ```
//! use trading_sessions::IdentifyTradingSession;
//!
//! let session_identifier = IdentifyTradingSession::new(1708574400);
//! assert_eq!(session_identifier.identify_trading_session(), "Tokyo");
//! ```
//!
//! ### SessionVerification
//!
//! ```
//! use trading_sessions::SessionVerification;
//!
//! let verifier = SessionVerification::new(1708574400, "Tokyo".to_string());
//! assert!(verifier.verify());
//! ```
//!
//! ### SessionColumn
//!
//! ```
//! use polars::prelude::*;
//! use trading_sessions::SessionColumn;
//!
//! let df = df! {
//!     "time" => [1708574400, 1708596000, 1708696800]
//! }.unwrap();
//! let lazy_frame = df.lazy();
//! let mut session_column = SessionColumn::new(lazy_frame);
//! session_column.apply_session_column();
//! let result_df = session_column.lazyframe.collect().unwrap();
//! assert_eq!(result_df.column("Session").unwrap().str_value(0).unwrap(), "Tokyo");
//! ```
//!
//! ## Return Types
//!
//! Successful operations return a string representing the trading session or a boolean indicating the verification result.
//! Errors are typically handled by the calling code and are dependent on the context in which these functions are used.
//!
//! ## Notes
//!
//! - The crate assumes all timestamps are in UTC.
//! - Daylight Saving Time is not considered in the current version.
//!
//! use polars::prelude::*;
//!
//! mod trading_sessions;
//!

use polars::prelude::*;

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