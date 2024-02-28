# sessions

The Sessions library, written in Rust, provides a robust solution for marking trading sessions on columnar timeseries data, specifically tailored for technical analysis. This library is designed to identify and verify trading sessions based on Unix timestamps, catering to the needs of financial data analysts and traders.

## Features

- **Identify Trading Sessions**: Determine the trading session (e.g., Tokyo, London, NewYork) based on the hour of the day in UTC, considering both UK and USA normal times.
- **Session Verification**: Verify if a given session string matches the trading session identified by a Unix timestamp.
- **Session Column Transformation**: Add a "Session" column to a `LazyFrame` based on Unix timestamps in a "time" column, facilitating easy integration with dataframes for further analysis.

## Usage

This library is particularly useful for financial data analysis, allowing users to seamlessly integrate trading session identification into their Rust-based data processing pipelines. It leverages the `polars` crate for efficient data manipulation, ensuring high performance and ease of use.

## Getting Started

To use the Sessions library in your project, add it as a dependency in your `Cargo.toml` file and explore the provided methods to identify and verify trading sessions in your timeseries data.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
