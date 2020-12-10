use chrono::Duration;
use std::fmt;

pub trait ChronoHumanDuration {
    type Displayer: fmt::Display;
    fn format_human(&self) -> Self::Displayer;
}

impl ChronoHumanDuration for Duration {
    type Displayer = Displayer;
    fn format_human(&self) -> Self::Displayer {
        Displayer { d: *self }
    }
}

pub struct Displayer {
    d: Duration,
}

impl fmt::Display for Displayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut wrote = false;
        let d = self.d;

        let months = d.num_weeks() / 4;
        if months > 0 {
            write!(f, "{} month{}", months, if months > 1 { "s" } else { "" })?;
            wrote = true;
        } else {
            let weeks = d.num_weeks();
            if weeks > 0 {
                write!(
                    f,
                    "{}{} week{}",
                    if wrote { ", " } else { "" },
                    weeks,
                    if weeks > 1 { "s" } else { "" }
                )?;
                wrote = true;
            } else {
                let days = d.num_days();
                if days > 0 {
                    write!(
                        f,
                        "{}{} day{}",
                        if wrote { ", " } else { "" },
                        days,
                        if days > 1 { "s" } else { "" }
                    )?;
                    wrote = true;
                } else {
                    let hours = d.num_hours();
                    if hours > 0 {
                        write!(
                            f,
                            "{}{} hour{}",
                            if wrote { ", " } else { "" },
                            hours,
                            if hours > 1 { "s" } else { "" }
                        )?;
                        wrote = true;
                    } else {
                        let minutes = d.num_minutes();
                        if minutes > 0 {
                            write!(
                                f,
                                "{}{} minute{}",
                                if wrote { ", " } else { "" },
                                minutes,
                                if minutes > 1 { "s" } else { "" }
                            )?;
                            wrote = true;
                        }
                    }
                }
            }
        }

        if wrote {
            write!(f, " ago")
        } else {
            write!(f, "just now")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ChronoHumanDuration;
    use chrono::Duration;

    #[test]
    fn it_works() {
        let d = Duration::weeks(2) + Duration::days(3) + Duration::hours(2) + Duration::minutes(20);
        assert_eq!(d.format_human().to_string(), "2 weeks ago");
        let d = Duration::weeks(1);
        assert_eq!(d.format_human().to_string(), "1 week ago");
        let d = Duration::days(2) + Duration::hours(2);
        assert_eq!(d.format_human().to_string(), "2 days ago");
        let d = Duration::days(1);
        assert_eq!(d.format_human().to_string(), "1 day ago");
        let d = Duration::hours(2);
        assert_eq!(d.format_human().to_string(), "2 hours ago");
        let d = Duration::hours(1);
        assert_eq!(d.format_human().to_string(), "1 hour ago");
        let d = Duration::minutes(2);
        assert_eq!(d.format_human().to_string(), "2 minutes ago");
        let d = Duration::minutes(1);
        assert_eq!(d.format_human().to_string(), "1 minute ago");
        let d = Duration::seconds(60);
        assert_eq!(d.format_human().to_string(), "1 minute ago");
        let d = Duration::seconds(59);
        assert_eq!(d.format_human().to_string(), "just now");
        let d = Duration::seconds(1);
        assert_eq!(d.format_human().to_string(), "just now");
    }
}
