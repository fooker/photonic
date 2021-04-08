use std::time::{Duration, Instant};

pub fn combine_opts<V, F>(v1: Option<V>, v2: Option<V>, f: F) -> Option<V>
where
    F: FnOnce(V, V) -> V,
{
    match (v1, v2) {
        (Some(v1), Some(v2)) => Some(f(v1, v2)),
        (Some(v1), None) => Some(v1),
        (None, Some(v2)) => Some(v2),
        (None, None) => None,
    }
}

#[derive(Clone)]
pub struct FrameStats {
    cycles: usize,

    min_time: Duration,
    max_time: Duration,
    sum_time: Duration,
}

impl FrameStats {
    pub fn new() -> Self {
        Self {
            cycles: 0,
            sum_time: Duration::new(0, 0),
            min_time: Duration::new(std::u64::MAX, 0),
            max_time: Duration::new(std::u64::MIN, 0),
        }
    }

    pub fn update(&mut self, duration: Duration, reset_cycles: usize) -> Option<Self> {
        self.cycles += 1;
        self.sum_time += duration;
        self.min_time = Duration::min(self.min_time, duration);
        self.max_time = Duration::max(self.max_time, duration);

        if self.cycles >= reset_cycles {
            let old = self.clone();

            self.cycles = 0;
            self.sum_time = Duration::new(0, 0);
            self.min_time = Duration::new(std::u64::MAX, 0);
            self.max_time = Duration::new(std::u64::MIN, 0);

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

pub struct FrameTimer {
    frame_time: Duration,

    frame_last: Instant,
    frame_curr: Instant,
}

impl FrameTimer {
    pub fn new(fps: usize) -> Self {
        Self {
            frame_time: Duration::from_secs(1) / fps as u32,
            frame_last: Instant::now(),
            frame_curr: Instant::now(),
        }
    }

    pub async fn next(&mut self) -> Duration {
        // Sleep until it's time to render next frame
        let next = self.frame_curr + self.frame_time;
        let curr = Instant::now();
        if next > curr {
            // TODO: This results in sleeping to long - find something more precise?
            tokio::time::sleep(next - curr).await;
        }

        // Remember current and previous frame start time
        self.frame_last = self.frame_curr;
        self.frame_curr = Instant::now();

        // Calculate differences between previous and current frame star times
        let duration = self.frame_curr - self.frame_last;

        return duration;
    }
}

pub struct TreeIterator<E, F, I>
where
    F: Fn(&E) -> I,
    I: Iterator<Item = E>,
{
    sprawl: F,
    stack: Vec<E>,
}

impl<E, F, I> TreeIterator<E, F, I>
where
    F: Fn(&E) -> I,
    I: Iterator<Item = E>,
{
    pub fn new(root: E, sprawl: F) -> Self {
        return Self {
            sprawl,
            stack: vec![root],
        };
    }
}

impl<E, F, I> Iterator for TreeIterator<E, F, I>
where
    F: Fn(&E) -> I,
    I: Iterator<Item = E>,
{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.stack.pop()?;
        self.stack.extend((self.sprawl)(&curr));
        return Some(curr);
    }
}
