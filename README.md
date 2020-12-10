# basic-human-duration

A very small crate to format durations for humans.

In comparison to the original [repo](https://github.com/derekdreery/chrono-human-duration)...
- this supports months
- this adjusts what is considered "just now" (before it was <60 minutes, now it is <60 seconds)
- has other minor adjustments

## Example

```rust
use basic_human_duration::ChronoHumanDuration;
use chrono::Duration;

let d = Duration::weeks(2) + Duration::days(3);
assert_eq!(d.format_human().to_string(), "2 weeks ago");
let d = Duration::seconds(20);
assert_eq!(d.format_human().to_string(), "just now");
```
