use crate::config::{Config, Service};

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

pub struct Timeouts {
    timeouts: HashMap<String, ServiceInfo>
}

impl Timeouts {
    pub fn from_config(cfg: Arc<Config>) -> Timeouts {
        let mut timeouts: HashMap<String, ServiceInfo> = HashMap::new();
        for (handle, service) in &cfg.services {
            timeouts.insert(handle.to_string(), ServiceInfo {
                next_timeout: next_timeout(service),
                state: ServiceState::Running,
                service: service.clone(),
            });
        }
        Timeouts {timeouts}
    }

    /// If the handle corresponds to a service, return Some(result), indicating whether the service
    /// was fine before, or if it just recovered. Otherwise return None.
    pub fn check_in(&mut self, handle: &str) -> Option<CheckInResult> {
        self.timeouts.get_mut(handle)
            .map(|info| {
                info.next_timeout = next_timeout(&info.service);
                let result = match info.state {
                    ServiceState::Running => CheckInResult::Normal,
                    ServiceState::Failed => CheckInResult::Recovery
                };
                info.state = ServiceState::Running;
                result
            })
    }

    pub fn name(&self, handle: &str) -> Option<String> {
        self.timeouts.get(handle).map(|info| info.service.name.clone())
    }

    pub fn interval(&self, handle: &str) -> Option<u32> {
        self.timeouts.get(handle).map(|info| info.service.interval)
    }

    pub fn get_failed(&mut self) -> Vec<Service> {
        let now = SystemTime::now();
        let mut result = Vec::new();
        for (handle, info) in &mut self.timeouts {
            if info.next_timeout < now {
                match info.state {
                    ServiceState::Running => {
                        result.push(info.service.clone());
                        info.state = ServiceState::Failed;
                    }
                    ServiceState::Failed => ()
                }
            }
        }
        result
    }
}

struct ServiceInfo {
    next_timeout: SystemTime,
    state: ServiceState,
    service: Service
}

pub enum CheckInResult {
    /// Checkin was done by a functioning service.
    Normal,
    /// This was the first checkin after at least one failure.
    Recovery
}

enum ServiceState {
    /// Everything OK since last report.
    Running,
    /// Service failed to check in.
    Failed
}

fn next_timeout(service: &Service) -> SystemTime {
    let duration = Duration::from_secs((service.interval * 60).into());
    SystemTime::now().checked_add(duration)
        .unwrap_or_else(|| panic!("SystemTime::checked_add failed. Time is weird here. But time is weird everywhere."))
}
