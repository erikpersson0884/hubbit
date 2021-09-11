use std::convert::TryFrom;

use anyhow::{bail, Result};
use chrono::NaiveDate;
use sqlx::PgPool;

use crate::models::StudyPeriod;

#[derive(Clone, Debug)]
pub struct StudyPeriodRepository {
  pool: PgPool,
}

impl StudyPeriodRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  pub async fn get_by_year_and_period(
    &self,
    year: i32,
    period: Period,
  ) -> Result<(NaiveDate, NaiveDate)> {
    let period_num: i32 = period.into();
    let study_period: StudyPeriod = match sqlx::query_as!(
      StudyPeriod,
      "
SELECT *
FROM study_periods
WHERE year = $1 AND period = $2
      ",
      year,
      period_num
    )
    .fetch_one(&self.pool)
    .await
    {
      Ok(study_period) => study_period,
      Err(_) => bail!("Something went wrong"),
    };

    Ok((study_period.start_date, study_period.end_date))
  }
}

#[derive(Clone, Copy, Debug)]
pub enum Period {
  LP1,
  LP2,
  LP3,
  LP4,
  Summer,
}

impl TryFrom<i32> for Period {
  type Error = anyhow::Error;

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(Self::LP1),
      1 => Ok(Self::LP2),
      2 => Ok(Self::LP3),
      3 => Ok(Self::LP4),
      4 => Ok(Self::Summer),
      _ => bail!("Invalid value, only 0-4 are valid"),
    }
  }
}

impl From<Period> for i32 {
  fn from(period: Period) -> Self {
    match period {
      Period::LP1 => 0,
      Period::LP2 => 1,
      Period::LP3 => 2,
      Period::LP4 => 3,
      Period::Summer => 4,
    }
  }
}
