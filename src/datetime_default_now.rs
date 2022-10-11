use std::ops::Deref;

use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

#[cfg(test)]
const NOW: &'static str = "2022/10/10 23:40:11.695164300";

/// # DateTime with current time as default.
///
/// ```should_panic
/// use chrono::{Local, TimeZone};
/// use datetime_default::DateTimeDefaultNow;
///
/// assert_eq!(
///     DateTimeDefaultNow::<Local>::default(),
///     Local.datetime_from_str("2022/10/10 23:40:11.695164300", "%Y/%m/%d %H:%M:%S%.9f").unwrap()
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTimeDefaultNow<Tz, const OFFSET_HOURS: i32 = 0>(DateTime<Tz>)
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy;

impl<const OFFSET_HOURS: i32> Default for DateTimeDefaultNow<FixedOffset, OFFSET_HOURS> {
    fn default() -> Self {
        Self(
            DateTimeDefaultNow::<Utc>::default()
                .with_timezone(&FixedOffset::east(OFFSET_HOURS * 3600)),
        )
    }
}

impl Default for DateTimeDefaultNow<Local, 0> {
    #[cfg(not(test))]
    fn default() -> Self {
        Self(Local::now())
    }

    #[cfg(test)]
    fn default() -> Self {
        Self(
            Local
                .datetime_from_str("2022/10/10 23:40:11.695164300", "%Y/%m/%d %H:%M:%S%.9f")
                .unwrap(),
        )
    }
}

impl Default for DateTimeDefaultNow<Utc, 0> {
    #[cfg(not(test))]
    fn default() -> Self {
        Self(Utc::now())
    }

    #[cfg(test)]
    fn default() -> Self {
        Self(Utc.datetime_from_str(NOW, "%Y/%m/%d %H:%M:%S%.9f").unwrap())
    }
}

impl<Tz, const OFFSET_HOURS: i32> Deref for DateTimeDefaultNow<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    type Target = DateTime<Tz>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Tz, const OFFSET_HOURS: i32> From<DateTime<Tz>> for DateTimeDefaultNow<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn from(datetime: DateTime<Tz>) -> Self {
        Self(datetime)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::cmp::PartialEq<DateTime<Tz>>
    for DateTimeDefaultNow<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn eq(&self, other: &DateTime<Tz>) -> bool {
        self.0.eq(other)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::cmp::PartialEq<DateTimeDefaultNow<Tz, OFFSET_HOURS>>
    for DateTime<Tz>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn eq(&self, other: &DateTimeDefaultNow<Tz, OFFSET_HOURS>) -> bool {
        self.eq(&other.0)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::cmp::PartialOrd<DateTime<Tz>>
    for DateTimeDefaultNow<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn partial_cmp(&self, other: &DateTime<Tz>) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::cmp::PartialOrd<DateTimeDefaultNow<Tz, OFFSET_HOURS>>
    for DateTime<Tz>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn partial_cmp(
        &self,
        other: &DateTimeDefaultNow<Tz, OFFSET_HOURS>,
    ) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {

    use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

    use crate::DateTimeDefaultNow;

    #[derive(Default)]
    struct Test {
        updated_at: DateTimeDefaultNow<Utc>,
    }

    fn print_datetime(datetime: &DateTime<Utc>) {
        println!("{datetime}")
    }

    #[test]
    fn default_derive_struct() {
        Test::default();
    }

    #[test]
    fn use_default_derive_struct() {
        let test = Test::default();

        print_datetime(&test.updated_at)
    }

    #[test]
    fn use_default_datetime() {
        let datetime = DateTimeDefaultNow::<Utc>::default();

        print_datetime(&datetime)
    }

    #[test]
    fn fixed_offset() {
        let datetime = DateTimeDefaultNow::<FixedOffset, 9>::default();
        println!("{datetime:?}");

        // assert!(false);
    }

    #[test]
    fn use_datetime_method() {
        let datetime = DateTimeDefaultNow::<Utc>::default();

        datetime.date();
    }

    #[test]
    fn convert_from() {
        let now = Local::now();
        let datetime = DateTimeDefaultNow::<Local>::from(now);

        assert_eq!(datetime, now)
    }

    #[test]
    fn convert_into() {
        let now = Local::now();
        let datetime: DateTimeDefaultNow<Local> = now.into();

        assert_eq!(datetime, now)
    }

    #[test]
    fn equation1() {
        let now = Local::now();
        let datetime: DateTimeDefaultNow<Local> = now.into();
        assert!(datetime == now)
    }

    #[test]
    fn equation2() {
        let now = Local::now();
        let datetime: DateTimeDefaultNow<Local> = now.into();
        assert!(now == datetime)
    }

    #[test]
    fn comparing1() {
        assert!(DateTimeDefaultNow::<Local>::default() <= Local::now())
    }

    #[test]
    fn comparing2() {
        assert!(
            Local
                .datetime_from_str("2022/10/10 00:00:00", "%Y/%m/%d %H:%M:%S")
                .unwrap()
                <= DateTimeDefaultNow::<Local>::default()
        )
    }
}
