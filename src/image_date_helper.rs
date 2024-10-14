

use std::{fs::File, io::BufReader, path::PathBuf};

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use exif::{Exif, In, Reader, Tag};

use crate::error::ImageParseError;

fn parse_exif_datestring(exif_string: String) -> Result<DateTime<Utc>, Box<dyn std::error::Error>>
{
    let image_date = exif_string.replace(" ", ":");
    let image_dates = image_date.split(":").collect::<Vec<_>>();

    let date = Utc.with_ymd_and_hms(image_dates[0].parse::<i32>()?, image_dates[1].parse::<u32>()?, image_dates[2].parse::<u32>()?, image_dates[3].parse::<u32>()?, image_dates[4].parse::<u32>()?, image_dates[5].parse::<u32>()?);
    let date = match date {
        chrono::LocalResult::Single(x) => x,
        _ => return Err(Box::new(ImageParseError::Exif))
    };

    return Ok(date)
    
}

fn get_file_name(path: &PathBuf) -> Result<String, ImageParseError>
{
    let stem = match path.file_stem() {
        Some(v) => v,
        None => return Err(ImageParseError::FileOpen)
    };

    let name = match stem.to_str() {
        Some(v) => v,
        None => return Err(ImageParseError::FileOpen)
    };

    return Ok(String::from(name));
}

fn get_date_exif(path: &PathBuf) -> Result<DateTime<Utc>, ImageParseError>
{
    let file = File::open(&path).map_err(|_| ImageParseError::FileOpen)?;
    let exif: Exif = Reader::new().read_from_container(&mut BufReader::new(&file)).map_err(|_| ImageParseError::Exif)?;

    let field: String = match exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) 
    {
        Some(v) => v.display_value().to_string(),
        None => return Err(ImageParseError::NoTag)
    };

    let field = field.replace("-", ":");

    let date = parse_exif_datestring(field).map_err(|_| ImageParseError::Exif)?;
    
    Ok(date)
    
}

fn get_date_whatsapp(file_name: String) -> Result<DateTime<Utc>, ImageParseError>
{
    let date_str = match file_name.split("-").nth(1)
    {
        Some(v) => v,
        None => return Err(ImageParseError::InvalidName),
    };

    let naive_date = NaiveDate::parse_from_str(date_str, "%Y%m%d").map_err(|_| ImageParseError::InvalidName)?;
    let naive_datetime = match naive_date.and_hms_opt(12, 0, 0)
    {
        Some(v) => v,
        None => return Err(ImageParseError::InvalidName),
    };
    let date_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc);

    Ok(date_time)
}

pub fn get_image_date(path: &PathBuf) -> Result<DateTime<Utc>, ImageParseError>
{

    let file_name = get_file_name(path)?;

    let date: DateTime<Utc>;
    
    if file_name.starts_with("IMG") && file_name.contains("WA")
    {
        date = get_date_whatsapp(file_name)?;
    } else {
        date = get_date_exif(path)?;
    }
    
    Ok(date)
}
