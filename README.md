# chrono-human-duration

Very small crate for format durations for humans.

In comparison to the original repo...
- supports months
- adjustes what is considered "just now" (before it was <60 minutes, now it is <60 seconds)
- other minor adjustments

## Example

```rust
use chrono_human_duration::ChronoHumanDuration;
use chrono::Duration;

let d = Duration::weeks(2) + Duration::days(3);
assert_eq!(d.format_human().to_string(), "2 weeks ago");
let d = Duration::seconds(20);
assert_eq!(d.format_human().to_string(), "just now");
```
