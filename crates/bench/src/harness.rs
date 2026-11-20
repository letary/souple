//! The timing: `std::time::Instant` (QueryPerformanceCounter on Windows, 100 ns ticks), a warm-up,
//! then samples until a time budget is spent, reported as median / p95 in µs. `yoga/bench.cpp`
//! runs the same loop (`steady_clock`, also QPC under MSVC's STL).
//!
//! An operation under `MIN_SAMPLE_US` is timed in a BATCH of calls per sample (the sample is the
//! batch's mean): a single call would be mostly timer resolution. Such a p95 is of batch means and
//! hides the tail; the `batch` field says when that happened.

use std::time::Instant;

pub const MIN_SAMPLE_US: f64 = 5.0;

#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub median: f64,
    pub p95: f64,
    pub mean: f64,
    pub min: f64,
    pub samples: usize,
    pub batch: usize,
}

impl Stats {
    pub fn from(values: &mut [f64], batch: usize) -> Stats {
        assert!(!values.is_empty());
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let at = |q: f64| values[((values.len() - 1) as f64 * q).round() as usize];
        Stats {
            median: at(0.5),
            p95: at(0.95),
            mean: values.iter().sum::<f64>() / values.len() as f64,
            min: values[0],
            samples: values.len(),
            batch,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "median": self.median, "p95": self.p95, "mean": self.mean, "min": self.min,
            "samples": self.samples, "batch": self.batch,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Budget {
    /// Seconds of samples per scenario, warm-up excluded.
    pub seconds: f64,
    pub min_samples: usize,
    pub max_samples: usize,
}

/// However expensive an operation, it gets this many samples; past 10 × the budget, no more.
pub const FLOOR_SAMPLES: usize = 3;

impl Budget {
    /// Another sample? `min_samples` of them — unless 10 × the budget is spent and `FLOOR_SAMPLES`
    /// are taken (an operation of seconds) — then more while the budget lasts.
    pub fn wants(&self, taken: usize, elapsed: f64) -> bool {
        let capped = taken >= FLOOR_SAMPLES && elapsed > 10.0 * self.seconds;
        (taken < self.min_samples && !capped) || (elapsed < self.seconds && taken < self.max_samples)
    }
}

pub fn elapsed_us(since: Instant) -> f64 {
    since.elapsed().as_secs_f64() * 1e6
}

/// Times a repeatable operation.
pub fn bench(budget: &Budget, mut op: impl FnMut()) -> Stats {
    // Warm-up: at least 3 calls and 20 ms; it also estimates the cost of one call.
    let start = Instant::now();
    let mut calls = 0u64;
    while calls < 3 || (start.elapsed().as_secs_f64() < 0.02 && calls < 10_000_000) {
        op();
        calls += 1;
    }
    let per_call = elapsed_us(start) / calls as f64;
    let batch = if per_call >= MIN_SAMPLE_US { 1 } else { ((MIN_SAMPLE_US / per_call).ceil() as usize).min(100_000) };

    let mut samples = Vec::new();
    let begin = Instant::now();
    while budget.wants(samples.len(), begin.elapsed().as_secs_f64()) {
        let t = Instant::now();
        for _ in 0..batch {
            op();
        }
        samples.push(elapsed_us(t) / batch as f64);
    }
    Stats::from(&mut samples, batch)
}

/// Keeps sampling while the budget lasts: `true` = take another sample.
pub struct Sampler {
    begin: Instant,
    taken: usize,
    budget: Budget,
}

impl Sampler {
    pub fn new(budget: Budget) -> Self {
        Self { begin: Instant::now(), taken: 0, budget }
    }

    pub fn more(&mut self) -> bool {
        let more = self.budget.wants(self.taken, self.begin.elapsed().as_secs_f64());
        self.taken += 1;
        more
    }
}
