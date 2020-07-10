// based on https://github.com/connerebbinghaus/rust-cache-control, licensed under MIT

use std::time::Duration;

/// How the data may be cached.
#[derive(Eq, PartialEq, Debug)]
pub enum Cachability {
    /// Any cache can cache this data.
    Public,
    /// Data cannot be cached in shared caches.
    Private,
    /// No one can cache this data.
    NoCache,
    /// Cache the data the first time, and use the cache from then on.
    OnlyIfCached,
}

/// Represents a Cache-Control header
#[derive(Eq, PartialEq, Debug, Default)]
pub struct CacheControl {
    pub cachability: Option<Cachability>,
    pub max_age: Option<Duration>,
    pub s_max_age: Option<Duration>,
    pub max_stale: Option<Duration>,
    pub min_fresh: Option<Duration>,
    pub must_revalidate: bool,
    pub proxy_revalidate: bool,
    pub immutable: bool,
    pub no_store: bool,
    pub no_transform: bool,
}

impl CacheControl {
    /// Parses the value of the Cache-Control header (i.e. everything after "Cache-Control:").
    pub fn from_value(value: &str) -> Option<CacheControl> {
        let mut ret = CacheControl::default();
        for token in value.split(',') {
            let mut key_value = token.split('=').map(str::trim);

            match key_value.next()? {
                "public" => ret.cachability = Some(Cachability::Public),
                "private" => ret.cachability = Some(Cachability::Private),
                "no-cache" => ret.cachability = Some(Cachability::NoCache),
                "only-if-cached" => ret.cachability = Some(Cachability::OnlyIfCached),
                "max-age" => {
                    let val = key_value.next()?.parse().ok()?;
                    ret.max_age = Some(Duration::from_secs(val));
                }
                "max-stale" => {
                    let val = key_value.next()?.parse().ok()?;
                    ret.max_stale = Some(Duration::from_secs(val));
                }
                "min-fresh" => {
                    let val = key_value.next()?.parse().ok()?;
                    ret.min_fresh = Some(Duration::from_secs(val));
                }
                "must-revalidate" => ret.must_revalidate = true,
                "proxy-revalidate" => ret.proxy_revalidate = true,
                "immutable" => ret.immutable = true,
                "no-store" => ret.no_store = true,
                "no-transform" => ret.no_transform = true,
                _ => (),
            };
        }
        Some(ret)
    }
}
