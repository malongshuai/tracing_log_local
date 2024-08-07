

## tracing_subscribe timer formatter use chrono

``` rust
/// specify the timezone `+08:00` for logging, 
/// and the default time format is `"%FT%T%.3f"` 
let local_timer = LocalTimer::new(chrono::FixedOffset::east_opt(8 * 3600).unwrap());

/// if you want to change the default time format, call `set_time_format()` 
/// let local_timer = local_timer.set_time_format("%FT%T%.3f");

tracing_subscriber::fmt()
      .with_timer(local_timer)
      .with_ansi(false)
      .init();
tracing::error!("hello, world");
```



