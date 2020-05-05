use chrono::prelude::*;

pub fn tomorrow(offset_hour: i32) -> DateTime<Utc> {
    tomorrow_from(&Utc::now(), offset_hour)
}

pub fn tomorrow_from(from: &DateTime<Utc>, offset_hour: i32) -> DateTime<Utc> {
    const HOUR: i32 = 3600;
    let now = from.with_timezone(&FixedOffset::east(offset_hour * HOUR));
    let tmrw = now.date().succ().and_hms(0, 0, 0);
    tmrw.with_timezone(&Utc)
}

#[test]
fn test_tomorrow_1() {
    let t = Utc.ymd(2020, 5, 5).and_hms(1, 23, 45);
    let r = tomorrow_from(&t, 9);
    assert_eq!(r, Utc.ymd(2020, 5, 5).and_hms(15, 0, 0));
}
#[test]
fn test_tomorrow_2() {
    let t = Utc.ymd(2020, 5, 5).and_hms(15, 23, 45);
    let r = tomorrow_from(&t, 9);
    assert_eq!(r, Utc.ymd(2020, 5, 6).and_hms(15, 0, 0));
}
#[test]
fn test_tomorrow_3() {
    let jp = FixedOffset::east(9 * 3600);
    let t = jp.ymd(2020, 5, 5).and_hms(0, 0, 0).with_timezone(&Utc);
    let r = tomorrow_from(&t, 9);
    assert_eq!(r, jp.ymd(2020, 5, 6).and_hms(0, 0, 0).with_timezone(&Utc));
}
#[test]
fn test_tomorrow_4() {
    let jp = FixedOffset::east(9 * 3600);
    let t = jp.ymd(2020, 5, 5).and_hms(23, 59, 59).with_timezone(&Utc);
    let r = tomorrow_from(&t, 9);
    assert_eq!(r, jp.ymd(2020, 5, 6).and_hms(0, 0, 0).with_timezone(&Utc));
}
