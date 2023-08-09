use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Represents a set of value for backoff strategy.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Retry {
    current: usize,
    max_retry: usize,
    backoff: Backoff,
}

impl Retry {
    pub fn new(max_retry: usize, backoff: Backoff) -> Retry {
        Retry {
            current: 0,
            max_retry,
            backoff,
        }
    }

    /// Current retry count.
    pub fn current(&self) -> usize {
        self.current
    }

    /// Max retry count.
    pub fn max_retry(&self) -> usize {
        self.max_retry
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
        if self.current > self.max_retry {
            None
        } else {
            let duration = self.backoff.delay_of(self.current - 1);
            Some((duration, self))
        }
    }
}
