use std::time::Duration;

pub struct Timer(Option<TickerImpl>);

struct TickerImpl {
    duration: Duration,
    remaining: Duration,
}

impl Timer {
    pub fn new(duration: Option<Duration>) -> Self {
        if let Some(duration) = duration {
            return Self(Some(TickerImpl {
                duration,
                remaining: duration,
            }));
        } else {
            return Self(None);
        }
    }

    pub fn update(&mut self, duration: &Duration) -> bool {
        if let Some(ref mut ticker) = self.0 {
            if ticker.remaining < *duration {
                ticker.remaining += ticker.duration - *duration;
                return true;
            } else {
                ticker.remaining -= *duration;
                return false;
            }
        } else {
            return false;
        }
    }
}
