use std::collections::VecDeque;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use lru::LruCache;
use rayon::ThreadPool;
use url::Url;

use crate::app::SIM_STEP;
use crate::overlay::{Log, LogData};

use super::{Level, LevelLoadError};

const SIM_STEP_SECS: f32 = SIM_STEP.as_secs_f32();
const TIMEOUT_SECS: f32 = 10.0;
const TIMEOUT_TICKS: u64 = (TIMEOUT_SECS / SIM_STEP_SECS).ceil() as u64;

enum LevelEntry {
    Loading(Option<JoinHandle<Result<Level, LevelLoadError>>>),
    Ready(Arc<Level>),
    Failed {
        error: Arc<LevelLoadError>,
        tick: u64,
    },
}

pub enum LevelCacheResult {
    Loading,
    Ready(Arc<Level>),
    Failed(Arc<LevelLoadError>),
}

pub struct LevelCache {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    asset_thread_pool: Arc<ThreadPool>,
    pending: VecDeque<Url>,
    cache: LruCache<Url, LevelEntry>,
}

pub struct LevelCacheNewParams {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub asset_thread_pool: Arc<ThreadPool>,
    pub capacity: usize,
}

impl LevelCache {
    pub fn new(params: LevelCacheNewParams) -> Self {
        Self {
            device: params.device,
            queue: params.queue,
            asset_thread_pool: params.asset_thread_pool,
            pending: VecDeque::new(),
            cache: LruCache::new(NonZeroUsize::new(params.capacity).unwrap()),
        }
    }

    pub fn get(&mut self, url: &Url, tick: u64) -> LevelCacheResult {
        let cached_result = match self.cache.get(url) {
            Some(LevelEntry::Ready(level)) => Some(LevelCacheResult::Ready(Arc::clone(level))),
            Some(LevelEntry::Loading(_)) => Some(LevelCacheResult::Loading),
            Some(LevelEntry::Failed {
                error,
                tick: failed_tick,
            }) if tick <= *failed_tick + TIMEOUT_TICKS => {
                Some(LevelCacheResult::Failed(Arc::clone(error)))
            }
            _ => None,
        };

        if let Some(result) = cached_result {
            return result;
        }

        self.cache.pop(url);

        let device = Arc::clone(&self.device);
        let queue = Arc::clone(&self.queue);
        let asset_thread_pool = Arc::clone(&self.asset_thread_pool);
        let url_clone = url.clone();
        let handle =
            thread::spawn(move || Level::load(url_clone, &device, &queue, &asset_thread_pool));

        self.cache
            .put(url.clone(), LevelEntry::Loading(Some(handle)));
        self.pending.push_back(url.clone());
        return LevelCacheResult::Loading;
    }

    pub fn update(&mut self, log: &mut Log, tick: u64) {
        let Some(url) = self.pending.front().cloned() else {
            return;
        };
        let Some(entry) = self.cache.get_mut(&url) else {
            return;
        };
        let LevelEntry::Loading(handle_opt) = entry else {
            panic!("pending queue contains URL with Ready entry");
        };

        if handle_opt.as_ref().is_some_and(|h| h.is_finished()) {
            let handle = handle_opt.take().unwrap();
            match handle.join().unwrap() {
                Ok(level) => {
                    let level_name = level.meta().name.to_string();
                    *entry = LevelEntry::Ready(Arc::new(level));
                    log.push(LogData::LoadSucceeded { name: level_name });
                }
                Err(error) => {
                    let error_message = error.to_string();
                    *entry = LevelEntry::Failed {
                        error: Arc::new(error),
                        tick,
                    };
                    log.push(LogData::LoadFailed {
                        url: url.to_string(),
                        error: error_message,
                    });
                }
            }
            self.pending.pop_front();
        }
    }

    pub fn clear(&mut self) {
        self.pending.clear();
        self.cache.clear();
    }
}
