# Trading Sessions
> A Rust crate for identifying and verifying trading sessions based on Unix timestamps.

## Getting Started
> Add the following to your `Cargo.toml`:
> ```toml
> [dependencies]
> trading_sessions = "0.1.0"
> ```

## Features
> - [`IdentifyTradingSession`](./struct.IdentifyTradingSession.html): Determine the trading session from a Unix timestamp.
> - [`SessionVerification`](./struct.SessionVerification.html): Verify if a given session string matches the identified trading session.
> - [`SessionColumn`](./struct.SessionColumn.html): Add a "Session" column to a `LazyFrame` based on Unix timestamps.

## Examples

### IdentifyTradingSession
> ```
> use trading_sessions::IdentifyTradingSession;
> 
> let session_identifier = IdentifyTradingSession::new(1708574400);
> assert_eq!(session_identifier.identify_trading_session(), "Tokyo");
> ```

### SessionVerification
> ```
> use trading_sessions::SessionVerification;
> 
> let verifier = SessionVerification::new(1708574400, "Tokyo".to_string());
> assert!(verifier.verify());
> ```

### SessionColumn
> ```
> use polars::prelude::*;
> use trading_sessions::SessionColumn;
> 
> let df = df! {
>     "time" => [1708574400, 1708596000, 1708696800]
> }.unwrap();
> let lazy_frame = df.lazy();
> let mut session_column = SessionColumn::new(lazy_frame);
> session_column.apply_session_column();
> let result_df = session_column.lazyframe.collect().unwrap();
> assert_eq!(result_df.column("Session").unwrap().str_value(0).unwrap(), "Tokyo");
> ```

## Return Types
> Successful operations return a string representing the trading session or a boolean indicating the verification result.
> Errors are typically handled by the calling code and are dependent on the context in which these functions are used.

## Notes
> - The crate assumes all timestamps are in UTC.
> - Daylight Saving Time is not considered in the current version.
