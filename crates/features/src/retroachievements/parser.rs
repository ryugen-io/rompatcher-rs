//! Simple JSON parser for RetroAchievements API responses

/// Parse simple RA JSON response: {"Success":true,"GameID":12345}
#[inline]
pub fn parse_game_id_response(json: &str) -> Result<Option<u32>, String> {
    // Fast path: check success field
    if !json.contains("\"Success\":true") {
        return Ok(None);
    }

    // Find GameID value - use bytes for faster digit scanning
    if let Some(start_pos) = json.find("\"GameID\":") {
        let bytes = json.as_bytes();
        let mut pos = start_pos + 9; // Skip "GameID":

        // Skip whitespace
        while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }

        // Find end of digits
        let digit_start = pos;
        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
            pos += 1;
        }

        // Parse number (no allocation - direct slice)
        if pos > digit_start
            && let Some(id) = json[digit_start..pos].parse::<u32>().ok()
            && id > 0
        {
            return Ok(Some(id));
        }
    }

    Ok(None)
}
