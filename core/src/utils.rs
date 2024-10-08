use std::collections::VecDeque;
use std::time::Duration;

pub fn combine_opts<V, F>(v1: Option<V>, v2: Option<V>, f: F) -> Option<V>
where F: FnOnce(V, V) -> V {
    match (v1, v2) {
        (Some(v1), Some(v2)) => Some(f(v1, v2)),
        (Some(v1), None) => Some(v1),
        (None, Some(v2)) => Some(v2),
        (None, None) => None,
    }
}

#[derive(Debug, Clone)]
pub struct FrameStats {
    cycles: usize,

    min_time: Duration,
    max_time: Duration,
    sum_time: Duration,
}

impl FrameStats {
    pub fn update(&mut self, duration: Duration) {
        self.cycles += 1;
        self.sum_time += duration;
        self.min_time = Duration::min(self.min_time, duration);
        self.max_time = Duration::max(self.max_time, duration);
    }

    pub fn reset(&mut self, reset_cycles: usize) -> Option<Self> {
        if self.cycles >= reset_cycles {
            let old = self.clone();

            self.cycles = 0;
            self.sum_time = Duration::new(0, 0);
            self.min_time = Duration::new(u64::MAX, 0);
            self.max_time = Duration::new(u64::MIN, 0);

            return Some(old);
        } else {
            return None;
        }
    }

    pub fn cycles(&self) -> usize {
        self.cycles
    }

    pub fn min_time(&self) -> Duration {
        self.min_time
    }

    pub fn max_time(&self) -> Duration {
        self.max_time
    }

    pub fn min_fps(&self) -> f64 {
        return 1.0f64 / self.max_time.as_secs_f64();
    }

    pub fn max_fps(&self) -> f64 {
        return 1.0f64 / self.min_time.as_secs_f64();
    }

    pub fn avg_fps(&self) -> f64 {
        return 1.0f64 / (self.sum_time.as_secs_f64() / self.cycles as f64);
    }
}

impl Default for FrameStats {
    fn default() -> Self {
        return Self {
            cycles: 0,
            sum_time: Duration::new(0, 0),
            min_time: Duration::new(u64::MAX, 0),
            max_time: Duration::new(u64::MIN, 0),
        };
    }
}

#[derive(Debug)]
pub struct FrameTimer {
    ticker: tokio::time::Interval,

    frame_last: tokio::time::Instant,
    frame_curr: tokio::time::Instant,
}

impl FrameTimer {
    pub fn new(fps: usize) -> Self {
        let frame_time = Duration::from_secs(1) / fps as u32;

        let start = tokio::time::Instant::now();

        Self {
            ticker: tokio::time::interval_at(start, frame_time),
            frame_last: start,
            frame_curr: start,
        }
    }

    pub async fn tick(&mut self) -> Duration {
        // Sleep until it's time to render next frame
        let now = self.ticker.tick().await;

        // Remember current and previous frame start time
        self.frame_last = self.frame_curr;
        self.frame_curr = now;

        // Calculate differences between previous and current frame star times
        let duration = self.frame_curr - self.frame_last;

        return duration;
    }
}

pub struct TreeIterator<'a, T, F, I>
where
    T: 'a,
    F: Fn(&'a T) -> I,
    I: Iterator<Item = &'a T>,
{
    sprawl: F,
    queue: VecDeque<&'a T>,
}

impl<'a, T, F, I> TreeIterator<'a, T, F, I>
where
    F: Fn(&'a T) -> I,
    I: Iterator<Item = &'a T>,
{
    pub fn new(t: &'a T, sprawl: F) -> Self {
        return Self {
            sprawl,
            queue: VecDeque::from([t]),
        };
    }
}

impl<'a, T, F, I> Iterator for TreeIterator<'a, T, F, I>
where
    F: Fn(&'a T) -> I,
    I: Iterator<Item = &'a T>,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t) = self.queue.pop_front() {
            self.queue.extend((self.sprawl)(t));
            return Some(t);
        } else {
            return None;
        }
    }
}
