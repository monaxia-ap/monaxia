use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Represents a set of value for backoff strategy.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Backoff {
    /// Constant backoff.
    Constant(Duration),

    /// Linear backoff.
    Linear { initial: Duration, delta: Duration },

    /// Exponential backoff.
    Exponential { initial: Duration, factor: f64 },
}

impl Backoff {
    /// Calculates delay duration for specified retry count.
    /// `retry` should be 0-based.
    pub fn delay_of(self, retry: usize) -> Duration {
        match self {
            Backoff::Constant(d) => d,
            Backoff::Linear { initial, delta } => initial + delta * retry as u32,
            Backoff::Exponential { initial, factor } => initial.mul_f64(factor.powf(retry as f64)),
        }
    }
}

/// Retry information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Retry {
    current: usize,
    max: usize,
    backoff: Backoff,
}

impl Retry {
    pub fn new(max: usize, backoff: Backoff) -> Retry {
        Retry {
            current: 0,
            max,
            backoff,
        }
    }

    /// Current retry count.
    pub fn current(&self) -> usize {
        self.current
    }

    /// Max retry count.
    pub fn max(&self) -> usize {
        self.max
    }

    /// Backoff strategy for the retry.
    pub fn backoff(&self) -> Backoff {
        self.backoff
    }

    /// Try to fetch next retry.
    /// If exceeded the max count, it will return `None`.
    /// Otherwise will be pair of next delay duration and `Retry`.
    pub fn retry(mut self) -> Option<(Duration, Retry)> {
        self.current += 1;
        if self.current > self.max {
            None
        } else {
            let duration = self.backoff.delay_of(self.current - 1);
            Some((duration, self))
        }
    }
}
