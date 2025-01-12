use chrono::NaiveDate;

pub fn iso8601_str_to_date(s: &str) -> Result<NaiveDate, ()> {
    let xs: Vec<u32> = s
        .split('-')
        .map(|s| s.parse::<u32>().map_err(|_| ()))
        .collect::<Result<Vec<_>, _>>()?;
    let year: u32 = *xs.get(0).ok_or(())?;
    let year: i32 = year.try_into().map_err(|_| ())?;
    let month = xs.get(1).ok_or(())?;
    let day = xs.get(2).ok_or(())?;
    let date = NaiveDate::from_ymd_opt(year, *month, *day).ok_or(())?;
    Ok(date)
}
