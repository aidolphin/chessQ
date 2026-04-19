use serde_json::Value;

const OPENING_EXPLORER_URL: &str = "https://explorer.lichess.ovh/masters";

pub fn best_opening_uci(fen: &str) -> Result<Option<String>, String> {
    let response = ureq::get(OPENING_EXPLORER_URL)
        .query("fen", fen)
        .query("moves", "12")
        .query("topGames", "0")
        .query("recentGames", "0")
        .call()
        .map_err(|error| format!("Lichess request failed: {error}"))?;

    let payload: Value = response
        .into_json()
        .map_err(|error| format!("Invalid Lichess JSON response: {error}"))?;

    Ok(parse_best_opening_uci(&payload))
}

fn parse_best_opening_uci(payload: &Value) -> Option<String> {
    payload
        .get("moves")
        .and_then(Value::as_array)
        .and_then(|moves| moves.first())
        .and_then(|mv| mv.get("uci"))
        .and_then(Value::as_str)
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::parse_best_opening_uci;
    use serde_json::json;

    #[test]
    fn parse_best_move_when_present() {
        let payload = json!({
            "moves": [
                { "uci": "e2e4", "white": 100, "draws": 20, "black": 80 }
            ]
        });
        assert_eq!(parse_best_opening_uci(&payload), Some("e2e4".to_string()));
    }

    #[test]
    fn parse_none_when_no_moves_exist() {
        let payload = json!({ "moves": [] });
        assert_eq!(parse_best_opening_uci(&payload), None);
    }
}
