use std::time;

const SECONDS_IN_YEAR: u64 = 31622400;
const SECONDS_IN_DAY: u64 = 86400;

const SECONDS_TO_1990: time::Duration = time::Duration::from_secs(631152000);

#[inline]
pub fn days_since_jan_1() -> u32
{
    (duration_since_1990().as_secs() % SECONDS_IN_YEAR / SECONDS_IN_DAY) as u32
}

#[inline]
pub fn years_since_1990() -> u32
{
    (duration_since_1990().as_secs() / SECONDS_IN_YEAR) as u32
}

#[inline]
fn duration_since_1990() -> time::Duration
{
    time::SystemTime::now()
        .duration_since(system_time_1990())
        .unwrap()
}

#[inline]
fn system_time_1990() -> time::SystemTime
{
    time::UNIX_EPOCH.checked_add(SECONDS_TO_1990).unwrap()
}

#[cfg(test)]
mod test
{
    use super::*;
    
    #[test]
    fn test_years_since_1990()
    {
        assert_eq!(years_since_1990(), 30)
    }
}