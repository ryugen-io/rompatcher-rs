//! RetroAchievements API client

use std::sync::Mutex;
use std::time::{Duration, Instant};

const RA_API_BASE: &str = "https://retroachievements.org";
const RA_USER_AGENT: &str = "rom-patcher-rs/0.1";
const RATE_LIMIT_MS: u64 = 500; // 500ms between requests

/// Rate limiter for API requests
struct RateLimiter {
    last_request: Option<Instant>,
}

impl RateLimiter {
    fn wait_if_needed(&mut self) {
        if let Some(last) = self.last_request {
            let elapsed = last.elapsed();
            let min_delay = Duration::from_millis(RATE_LIMIT_MS);

            if elapsed < min_delay {
                std::thread::sleep(min_delay - elapsed);
            }
        }
        self.last_request = Some(Instant::now());
    }
}

static RATE_LIMITER: Mutex<RateLimiter> = Mutex::new(RateLimiter { last_request: None });

/// API response for game ID lookup
#[derive(serde::Deserialize, Debug)]
pub struct GameIdResponse {
    #[serde(rename = "Success")]
    pub success: bool,
    #[serde(rename = "GameID")]
    pub game_id: u32,
}

/// Look up game ID by MD5 hash
pub fn lookup_game_by_hash(md5_hash: &str) -> Result<Option<u32>, String> {
    // Rate limit
    RATE_LIMITER
        .lock()
        .map_err(|e| format!("Rate limiter error: {}", e))?
        .wait_if_needed();

    // Build URL
    let url = format!("{}/dorequest.php?r=gameid&m={}", RA_API_BASE, md5_hash);

    // Make request
    let response = ureq::get(&url)
        .set("User-Agent", RA_USER_AGENT)
        .call()
        .map_err(|e| format!("API request failed: {}", e))?;

    // Parse response
    let game_response: GameIdResponse = response
        .into_json()
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if game_response.success && game_response.game_id > 0 {
        Ok(Some(game_response.game_id))
    } else {
        Ok(None)
    }
}

/// Get RetroAchievements game URL
pub fn game_url(game_id: u32) -> String {
    format!("{}/game/{}", RA_API_BASE, game_id)
}
