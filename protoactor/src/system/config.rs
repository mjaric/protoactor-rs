use std::time::Duration;

#[derive(Debug)]
pub struct ActorSystemConfig {
    pub(super) dead_letter_throttle_interval: Duration,
    pub(super) dead_letter_throttle_count: i32,
    pub(super) dead_letter_request_logging: bool,
    pub(super) dead_letter_response_logging: bool,
    pub(super) developer_supervision_logging: bool,
    pub(super) metrics_enabled: bool,
    pub(super) shared_futures: bool,
    pub(super) shared_futures_size: usize,
    pub(super) actor_request_timeout: Duration,
}

impl Default for ActorSystemConfig {
    fn default() -> Self {
        Self {
            dead_letter_throttle_interval: Duration::from_secs(1),
            dead_letter_throttle_count: 3,
            dead_letter_request_logging: true,
            dead_letter_response_logging: true,
            developer_supervision_logging: true,
            metrics_enabled: false,
            shared_futures: true,
            shared_futures_size: 5000,
            actor_request_timeout: Duration::from_secs(5),
        }
    }
}

impl ActorSystemConfig {
    pub fn setup() -> Self {
        Default::default()
    }

    ///  The interval used to trigger throttling of deadletter message logs.
    pub fn with_dead_letter_throttle_interval(self, interval: Duration) -> Self {
        Self {
            dead_letter_throttle_interval: interval,
            ..self
        }
    }

    /// The counter used to trigger throttling of deadletter message logs
    /// DeadLetter throttling triggers when there are `dead_letter_throttle_count` deadletters in
    /// `dead_letter_throttle_interval` time.
    pub fn with_dead_letter_throttle_count(self, count: i32) -> Self {
        Self {
            dead_letter_throttle_count: count,
            ..self
        }
    }
    /// Enables logging for DeadLetter responses in request/request_async
    /// When disabled, the requesting code is responsible for logging manually
    pub fn with_dead_letter_request_logging(self, enabled: bool) -> Self {
        Self {
            dead_letter_request_logging: enabled,
            ..self
        }
    }

    /// Enables SharedFutures
    ///
    /// SharedFutures allows the ActorSystem to avoid registering a new temporary process
    /// for each request instead registering a SharedFuture that can handle multiple requests
    /// internally.
    ///
    /// # Arguments
    /// * `size` - Shared futures size
    pub fn with_with_shared_futures(self, size: usize) -> Self {
        Self {
            shared_futures_size: size,
            shared_futures: true,
            ..self
        }
    }

    /// Developer debugging feature, enables extended logging for actor supervision failures
    /// # Arguments
    /// * `enabled` - Set `true` to enable supervision logging
    pub fn with_developer_supervision_logging(self, enabled: bool) -> Self {
        Self {
            developer_supervision_logging: enabled,
            ..self
        }
    }

    /// Enables actor metrics. Set to true if you want to export the metrics
    /// with OpenTelemetry exporters.
    pub fn with_metrics(self, enabled: bool) -> Self {
        Self {
            metrics_enabled: enabled,
            ..self
        }
    }

    pub fn with_actor_request_timeout(self, timeout: Duration) -> Self {
        Self {
            actor_request_timeout: timeout,
            ..self
        }
    }
    /// Enables logging for DeadLetter responses in request/request_async (responses returned
    /// from DeadLetter to original sender).
    pub fn with_dead_letter_response_logging(self, enabled: bool) -> Self {
        Self {
            dead_letter_response_logging: enabled,
            ..self
        }
    }
}
