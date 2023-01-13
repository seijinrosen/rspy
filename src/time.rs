//! [Python の time モジュール](https://docs.python.org/ja/3/library/time.html) に対応するモジュール。

use std::{thread, time::Duration};

/// 指定した秒数、スリープする。
///
/// # Examples
///
/// ```
/// use rspy::time::sleep;
/// use std::time::{Duration, Instant};
///
/// let now = Instant::now();
///
/// sleep(1);
///
/// assert!(now.elapsed() >= Duration::from_secs(1));
/// ```
pub fn sleep(secs: u64) {
    thread::sleep(Duration::from_secs(secs));
}

#[cfg(test)]
mod tests {
    use crate as rspy;
    use rspy::time::sleep;
    use std::time::{Duration, Instant};

    #[test]
    fn sleep_works() {
        let now = Instant::now();
        sleep(1);
        assert!(now.elapsed() >= Duration::from_secs(1));
    }
}
