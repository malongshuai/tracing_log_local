use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

/// tracing_subscriber timer format with chrono crate
///
/// ```ignore
/// let local_timer = LocalTimer::new(chrono::FixedOffset::east_opt(8 * 3600).unwrap())
///     .set_time_format("%FT%T%.3f");
/// tracing_subscriber::fmt()
///       .with_timer(local_timer)
///       .with_ansi(false)
///       .init();
/// tracing::error!("hello, world");
/// ```
pub struct LocalTimer {
    offset: chrono::FixedOffset,
    _time_format: &'static str,
}

impl LocalTimer {
    /// default time format: `%FT%T%.3f`
    ///
    /// change the default time format by calling `set_time_format()`
    pub fn new(offset: chrono::FixedOffset) -> Self {
        Self {
            offset,
            _time_format: "%FT%T%.3f",
        }
    }
    pub fn set_time_format(mut self, format: &'static str) -> Self {
        self._time_format = format;
        self
    }
}

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let now = chrono::Utc::now().with_timezone(&self.offset);
        write!(w, "{}", now.format(self._time_format))
    }
}

#[cfg(test)]
mod tt {

    #[test]
    fn t() {
        use super::*;

        let local_timer = LocalTimer::new(chrono::FixedOffset::east_opt(8 * 3600).unwrap())
            .set_time_format("%FT%T%.3f");
        tracing_subscriber::fmt()
            .with_timer(local_timer)
            .with_file(false)
            .with_target(true)
            .with_line_number(true)
            .with_max_level(tracing::Level::INFO)
            .with_ansi(true)
            .init();

        tracing::error!("hello, world");
    }
}
