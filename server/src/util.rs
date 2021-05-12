use diesel::pg::data_types::PgTimestamp;

const TIMESTAMP_2000: u64 = 946684800;

pub fn pgtimestamp_to_epoch_seconds(pg: PgTimestamp) -> u64 {
    // Default is measured in microseconds so let's divide by 1,000,000
    let seconds_since_2000 = (pg.0 / 1_000_000) as u64;
    // Add the number of seconds from unix epoch to 2000 1st jan.
    TIMESTAMP_2000 + seconds_since_2000
}

pub fn epoch_seconds_to_pgtimestamp(epoch: u64) -> PgTimestamp {
    let seconds_since_2000 = epoch - TIMESTAMP_2000;
    PgTimestamp((seconds_since_2000 * 1_000_000) as i64)
}
