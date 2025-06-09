use jiff::civil::Date;

pub fn format_degiro_date(date: &Date) -> String {
    format!("{:04}/{:02}/{:02}", date.year(), date.month(), date.day())
}
