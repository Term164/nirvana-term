use chrono::TimeZone;

pub(crate) fn parse_time(input: &str) -> anyhow::Result<i64> {
    let now = chrono::Local::now();
    let today = now.date_naive();

    let parsed = if input.contains('-') {
        chrono::NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M").ok()
    } else {
        chrono::NaiveTime::parse_from_str(input, "%H:%M")
            .ok()
            .map(|t| today.and_time(t))
    };

    let dt = parsed.ok_or_else(|| {
        anyhow::anyhow!(
            "invalid time format: '{}'. Use HH:MM or YYYY-MM-DD HH:MM",
            input
        )
    })?;
    let local = chrono::Local
        .from_local_datetime(&dt)
        .single()
        .ok_or_else(|| anyhow::anyhow!("ambiguous or invalid local time"))?;

    Ok(local.timestamp())
}
