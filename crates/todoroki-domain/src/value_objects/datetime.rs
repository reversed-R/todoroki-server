use chrono::{Datelike, TimeZone, Timelike};

use crate::{value_object, value_objects::error::ErrorCode};

value_object!(DateTime(chrono::DateTime<chrono::Utc>));

impl DateTime {
    pub fn now() -> Self {
        Self(chrono::Utc::now())
    }
}

impl TryFrom<String> for DateTime {
    type Error = DateTimeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(
            chrono::DateTime::parse_from_rfc3339(&value)
                .map_err(|_| DateTimeError::InvalidDateTimeString(value))?
                .with_timezone(&chrono::Utc),
        ))
    }
}

pub enum DateTimeError {
    InvalidDateTimeString(String),
    InvalidTime(u32, u32, u32),
    InvalidMonthlyTime(u8, u32, u32, u32),
    InvalidEpochDate(i32, u32, u32),
}

impl From<DateTimeError> for ErrorCode {
    fn from(value: DateTimeError) -> Self {
        match value {
            DateTimeError::InvalidDateTimeString(s) => {
                Self::InvalidDateTimeFormat(format!("invalid-datetime-string; string={s}"))
            }
            DateTimeError::InvalidTime(hour, min, sec) => Self::InvalidDateTimeFormat(format!(
                "invalid-time; hour={hour}; minute={min}; second={sec}"
            )),
            DateTimeError::InvalidMonthlyTime(date, hour, min, sec) => {
                Self::InvalidDateTimeFormat(format!(
                    "invalid-monthly-time; date={date}; hour={hour}; minute={min}; second={sec}"
                ))
            }
            DateTimeError::InvalidEpochDate(year, month, date) => Self::InvalidDateTimeFormat(
                format!("invalid-epoch-date; year={year}; month={month}; date={date}"),
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Time(chrono::NaiveTime);

impl Time {
    pub fn try_new(hour: u32, min: u32, sec: u32) -> Result<Self, DateTimeError> {
        Ok(Self(
            chrono::NaiveTime::from_hms_opt(hour, min, sec)
                .ok_or(DateTimeError::InvalidTime(hour, min, sec))?,
        ))
    }

    pub fn value(self) -> chrono::NaiveTime {
        self.0
    }
}

impl From<Time> for DateTime {
    fn from(value: Time) -> Self {
        Self(
            chrono::Utc
                .with_ymd_and_hms(
                    1970,
                    1,
                    1,
                    value.0.hour(),
                    value.0.minute(),
                    value.0.second(),
                )
                .unwrap(),
        )
    }
}

impl TryFrom<DateTime> for Time {
    type Error = DateTimeError;

    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
        if value.0.year() == 1970 && value.0.month() == 1 && value.0.day() == 1 {
            Ok(Self(
                chrono::NaiveTime::from_hms_opt(value.0.hour(), value.0.minute(), value.0.second())
                    .unwrap(),
            ))
        } else {
            Err(DateTimeError::InvalidEpochDate(
                value.0.year(),
                value.0.month(),
                value.0.day(),
            ))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeeklyTime {
    weekday: chrono::Weekday,
    time: chrono::NaiveTime,
}

impl WeeklyTime {
    pub fn try_new(
        weekday: chrono::Weekday,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> Result<Self, DateTimeError> {
        Ok(Self {
            weekday,
            time: chrono::NaiveTime::from_hms_opt(hour, min, sec)
                .ok_or(DateTimeError::InvalidTime(hour, min, sec))?,
        })
    }
}

impl From<WeeklyTime> for DateTime {
    fn from(value: WeeklyTime) -> Self {
        Self(
            chrono::Utc
                .with_ymd_and_hms(
                    1970,
                    1,
                    5 + value.weekday.num_days_from_monday(),
                    value.time.hour(),
                    value.time.minute(),
                    value.time.second(),
                )
                .unwrap(),
        )
    }
}

impl TryFrom<DateTime> for WeeklyTime {
    type Error = DateTimeError;

    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
        if value.0.year() == 1970 && value.0.month() == 1 && (5..12).contains(&value.0.day()) {
            let weekday = match value.0.day() - 5 {
                0 => Some(chrono::Weekday::Mon),
                1 => Some(chrono::Weekday::Tue),
                2 => Some(chrono::Weekday::Wed),
                3 => Some(chrono::Weekday::Thu),
                4 => Some(chrono::Weekday::Fri),
                5 => Some(chrono::Weekday::Sat),
                6 => Some(chrono::Weekday::Sun),
                _ => None,
            }
            .unwrap();

            Ok(Self {
                weekday,
                time: chrono::NaiveTime::from_hms_opt(
                    value.0.hour(),
                    value.0.minute(),
                    value.0.second(),
                )
                .unwrap(),
            })
        } else {
            Err(DateTimeError::InvalidEpochDate(
                value.0.year(),
                value.0.month(),
                value.0.day(),
            ))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonthlyTime {
    date: u8, // 1 ~ 31
    time: chrono::NaiveTime,
}

impl MonthlyTime {
    pub fn try_new(date: u8, hour: u32, min: u32, sec: u32) -> Result<Self, DateTimeError> {
        let date = if (1..=31).contains(&date) {
            Ok(date)
        } else {
            Err(DateTimeError::InvalidMonthlyTime(date, hour, min, sec))
        }?;

        Ok(Self {
            date,
            time: chrono::NaiveTime::from_hms_opt(hour, min, sec)
                .ok_or(DateTimeError::InvalidMonthlyTime(date, hour, min, sec))?,
        })
    }
}

impl From<MonthlyTime> for DateTime {
    fn from(value: MonthlyTime) -> Self {
        Self(
            chrono::Utc
                .with_ymd_and_hms(
                    1970,
                    1,
                    value.date as u32,
                    value.time.hour(),
                    value.time.minute(),
                    value.time.second(),
                )
                .unwrap(),
        )
    }
}

impl TryFrom<DateTime> for MonthlyTime {
    type Error = DateTimeError;

    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
        if value.0.year() == 1970 && value.0.month() == 1 && (1..31).contains(&value.0.day()) {
            Ok(Self {
                date: value.0.day() as u8,
                time: chrono::NaiveTime::from_hms_opt(
                    value.0.hour(),
                    value.0.minute(),
                    value.0.second(),
                )
                .unwrap(),
            })
        } else {
            Err(DateTimeError::InvalidEpochDate(
                value.0.year(),
                value.0.month(),
                value.0.day(),
            ))
        }
    }
}
