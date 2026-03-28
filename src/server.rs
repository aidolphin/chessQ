use crate::ai::evaluation::Personality;
use crate::ai::search::AI;
use crate::engine::bitboard::Square;
use crate::engine::move_gen::{Color, Move, MoveGenerator, PieceType};
use crate::state::game_state::GameState;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;

pub fn run() -> io::Result<()> {
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "4173".to_string());
    let address = format!("{host}:{port}");
    let listener = TcpListener::bind(&address)?;

    println!("ChessQ web is ready at http://{address}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(error) = handle_connection(stream) {
                        eprintln!("request failed: {error}");
                    }
                });
            }
            Err(error) => eprintln!("connection failed: {error}"),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0u8; 16 * 1024];
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read == 0 {
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let request_line = request.lines().next().unwrap_or_default();
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or_default();
    let target = parts.next().unwrap_or("/");

    if method != "GET" {
        return write_response(
            &mut stream,
            "405 Method Not Allowed",
            "text/plain; charset=utf-8",
            b"Method not allowed",
        );
    }

    let (path, query) = split_target(target);
    if path.starts_with("/api/") {
        return handle_api(&mut stream, path, &query);
    }

    serve_static(&mut stream, path)
}

fn split_target(target: &str) -> (&str, String) {
    if let Some((path, query)) = target.split_once('?') {
        (path, query.to_string())
    } else {
        (target, String::new())
    }
}

fn handle_api(stream: &mut TcpStream, path: &str, query: &str) -> io::Result<()> {
    let params = parse_query(query);
    let generator = MoveGenerator::new();

    match path {
        "/api/new" => {
            let state = GameState::initial();
            let body = snapshot_json(&state, &generator, None);
            write_response(stream, "200 OK", "application/json; charset=utf-8", body.as_bytes())
        }
        "/api/state" => {
            let Some(fen) = params.get("fen") else {
                return api_error(stream, "Missing fen parameter");
            };

            match GameState::from_fen(fen) {
                Ok(state) => {
                    let body = snapshot_json(&state, &generator, None);
                    write_response(stream, "200 OK", "application/json; charset=utf-8", body.as_bytes())
                }
                Err(error) => api_error(stream, &error),
            }
        }
        "/api/legal-moves" => {
            let Some(fen) = params.get("fen") else {
                return api_error(stream, "Missing fen parameter");
            };
            let Some(from_coord) = params.get("from") else {
                return api_error(stream, "Missing from parameter");
            };

            let state = match GameState::from_fen(fen) {
                Ok(state) => state,
                Err(error) => return api_error(stream, &error),
            };
            let from = match coord_to_square(from_coord) {
                Ok(square) => square,
                Err(error) => return api_error(stream, &error),
            };

            let moves: Vec<Move> = generator
                .generate_moves(&state)
                .into_iter()
                .filter(|mv| mv.from == from)
                .collect();

            let body = format!("{{\"moves\":{}}}", moves_json(&moves));
            write_response(stream, "200 OK", "application/json; charset=utf-8", body.as_bytes())
        }
        "/api/move" => {
            let Some(fen) = params.get("fen") else {
                return api_error(stream, "Missing fen parameter");
            };
            let Some(from_coord) = params.get("from") else {
                return api_error(stream, "Missing from parameter");
            };
            let Some(to_coord) = params.get("to") else {
                return api_error(stream, "Missing to parameter");
            };

            let state = match GameState::from_fen(fen) {
                Ok(state) => state,
                Err(error) => return api_error(stream, &error),
            };
            let from = match coord_to_square(from_coord) {
                Ok(square) => square,
                Err(error) => return api_error(stream, &error),
            };
            let to = match coord_to_square(to_coord) {
                Ok(square) => square,
                Err(error) => return api_error(stream, &error),
            };
            let promotion = match params.get("promotion") {
                Some(value) => match promotion_from_code(value) {
                    Ok(piece) => Some(piece),
                    Err(error) => return api_error(stream, &error),
                },
                None => None,
            };

            let legal_move = generator
                .generate_moves(&state)
                .into_iter()
                .find(|mv| mv.from == from && mv.to == to && mv.promotion == promotion);

            let Some(legal_move) = legal_move else {
                return api_error(stream, "Illegal move");
            };

            match GameState::make_move(&state, &legal_move) {
                Ok(next_state) => {
                    let notation = move_notation(&legal_move, &next_state, &generator);
                    let snapshot = snapshot_json(&next_state, &generator, Some(&legal_move));
                    let body = format!(
                        "{{\"moveNotation\":{},\"snapshot\":{}}}",
                        json_string(&notation),
                        snapshot
                    );
                    write_response(stream, "200 OK", "application/json; charset=utf-8", body.as_bytes())
                }
                Err(error) => api_error(stream, &error),
            }
        }
        "/api/ai-move" => {
            let Some(fen) = params.get("fen") else {
                return api_error(stream, "Missing fen parameter");
            };

            let state = match GameState::from_fen(fen) {
                Ok(state) => state,
                Err(error) => return api_error(stream, &error),
            };

            let personality = match personality_from_code(
                params
                    .get("personality")
                    .map(String::as_str)
                    .unwrap_or("aggressive"),
            ) {
                Ok(personality) => personality,
                Err(error) => return api_error(stream, &error),
            };

            let thinking_ms = params
                .get("ms")
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or(700)
                .clamp(200, 3000);

            let mut ai = AI::new(personality);
            let search_result = ai.find_best_move(&state, std::time::Duration::from_millis(thinking_ms));
            let Some(best_move) = search_result.best_move else {
                return api_error(stream, "No legal AI move");
            };

            match GameState::make_move(&state, &best_move) {
                Ok(next_state) => {
                    let notation = move_notation(&best_move, &next_state, &generator);
                    let snapshot = snapshot_json(&next_state, &generator, Some(&best_move));
                    let body = format!(
                        "{{\"moveNotation\":{},\"snapshot\":{},\"evaluation\":{},\"depth\":{}}}",
                        json_string(&notation),
                        snapshot,
                        search_result.evaluation,
                        search_result.depth_reached
                    );
                    write_response(stream, "200 OK", "application/json; charset=utf-8", body.as_bytes())
                }
                Err(error) => api_error(stream, &error),
            }
        }
        _ => write_response(
            stream,
            "404 Not Found",
            "text/plain; charset=utf-8",
            b"Not found",
        ),
    }
}

fn serve_static(stream: &mut TcpStream, path: &str) -> io::Result<()> {
    let web_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
    let relative = if path == "/" { "index.html" } else { path.trim_start_matches('/') };

    if relative.contains("..") || relative.contains('\\') {
        return write_response(
            stream,
            "403 Forbidden",
            "text/plain; charset=utf-8",
            b"Forbidden",
        );
    }

    let file_path = web_root.join(relative);
    if !file_path.starts_with(&web_root) || !file_path.is_file() {
        return write_response(
            stream,
            "404 Not Found",
            "text/plain; charset=utf-8",
            b"Not found",
        );
    }

    let content = fs::read(&file_path)?;
    let mime_type = mime_type_for(&file_path);
    write_response(stream, "200 OK", mime_type, &content)
}

fn mime_type_for(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("svg") => "image/svg+xml; charset=utf-8",
        _ => "application/octet-stream",
    }
}

fn parse_query(query: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }

        let (raw_key, raw_value) = pair.split_once('=').unwrap_or((pair, ""));
        params.insert(percent_decode(raw_key), percent_decode(raw_value));
    }

    params
}

fn percent_decode(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let bytes = value.as_bytes();
    let mut index = 0usize;

    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                output.push(' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                let hex = &value[index + 1..index + 3];
                if let Ok(decoded) = u8::from_str_radix(hex, 16) {
                    output.push(decoded as char);
                    index += 3;
                } else {
                    output.push('%');
                    index += 1;
                }
            }
            byte => {
                output.push(byte as char);
                index += 1;
            }
        }
    }

    output
}

fn snapshot_json(state: &GameState, generator: &MoveGenerator, last_move: Option<&Move>) -> String {
    let status = status_for_state(state, generator);
    let last_move_json = match last_move {
        Some(mv) => format!(
            "{{\"from\":{},\"to\":{}}}",
            json_string(&square_to_coord(mv.from)),
            json_string(&square_to_coord(mv.to))
        ),
        None => "null".to_string(),
    };

    format!(
        "{{\"board\":{},\"fen\":{},\"turn\":{},\"status\":{{\"phase\":{},\"winner\":{},\"message\":{}}},\"gameOver\":{},\"lastMove\":{}}}",
        board_json(state),
        json_string(&state.to_fen()),
        json_string(color_code(state.side_to_move)),
        json_string(status.phase),
        option_json(status.winner.map(color_code)),
        json_string(&status.message),
        if status.game_over { "true" } else { "false" },
        last_move_json
    )
}

fn board_json(state: &GameState) -> String {
    let mut cells = Vec::with_capacity(64);

    for rank in (0..8).rev() {
        for file in 0..8 {
            let square = Square::from_index(file + rank * 8).unwrap();
            match state.piece_at(square) {
                Some((color, piece)) => cells.push(json_string(piece_code(color, piece))),
                None => cells.push("null".to_string()),
            }
        }
    }

    format!("[{}]", cells.join(","))
}

fn moves_json(moves: &[Move]) -> String {
    let payload: Vec<String> = moves
        .iter()
        .map(|mv| {
            format!(
                "{{\"from\":{},\"to\":{},\"promotion\":{},\"isCastle\":{},\"isEnPassant\":{}}}",
                json_string(&square_to_coord(mv.from)),
                json_string(&square_to_coord(mv.to)),
                option_json(mv.promotion.map(promotion_code)),
                if mv.is_castle { "true" } else { "false" },
                if mv.is_en_passant { "true" } else { "false" }
            )
        })
        .collect();

    format!("[{}]", payload.join(","))
}

fn move_notation(mv: &Move, next_state: &GameState, generator: &MoveGenerator) -> String {
    let mut notation = mv.algebraic();
    let status = status_for_state(next_state, generator);
    if status.phase == "checkmate" {
        notation.push('#');
    } else if status.phase == "check" {
        notation.push('+');
    }
    notation
}

struct StatusData {
    phase: &'static str,
    winner: Option<Color>,
    message: String,
    game_over: bool,
}

fn status_for_state(state: &GameState, generator: &MoveGenerator) -> StatusData {
    let legal_moves = generator.generate_moves(state);
    let in_check = state.is_check();

    if legal_moves.is_empty() && in_check {
        let winner = state.side_to_move.opposite();
        return StatusData {
            phase: "checkmate",
            winner: Some(winner),
            message: format!("{} wins by checkmate.", color_name(winner)),
            game_over: true,
        };
    }

    if legal_moves.is_empty() {
        return StatusData {
            phase: "stalemate",
            winner: None,
            message: "Draw by stalemate.".to_string(),
            game_over: true,
        };
    }

    if in_check {
        return StatusData {
            phase: "check",
            winner: None,
            message: format!("{} to move and in check.", color_name(state.side_to_move)),
            game_over: false,
        };
    }

    StatusData {
        phase: "playing",
        winner: None,
        message: format!("{} to move.", color_name(state.side_to_move)),
        game_over: false,
    }
}

fn color_name(color: Color) -> &'static str {
    match color {
        Color::White => "White",
        Color::Black => "Black",
    }
}

fn color_code(color: Color) -> &'static str {
    match color {
        Color::White => "w",
        Color::Black => "b",
    }
}

fn piece_code(color: Color, piece: PieceType) -> &'static str {
    match (color, piece) {
        (Color::White, PieceType::Pawn) => "wp",
        (Color::White, PieceType::Knight) => "wn",
        (Color::White, PieceType::Bishop) => "wb",
        (Color::White, PieceType::Rook) => "wr",
        (Color::White, PieceType::Queen) => "wq",
        (Color::White, PieceType::King) => "wk",
        (Color::Black, PieceType::Pawn) => "bp",
        (Color::Black, PieceType::Knight) => "bn",
        (Color::Black, PieceType::Bishop) => "bb",
        (Color::Black, PieceType::Rook) => "br",
        (Color::Black, PieceType::Queen) => "bq",
        (Color::Black, PieceType::King) => "bk",
    }
}

fn promotion_code(piece: PieceType) -> &'static str {
    match piece {
        PieceType::Queen => "q",
        PieceType::Rook => "r",
        PieceType::Bishop => "b",
        PieceType::Knight => "n",
        PieceType::Pawn | PieceType::King => unreachable!(),
    }
}

fn promotion_from_code(code: &str) -> Result<PieceType, String> {
    match code {
        "q" | "Q" => Ok(PieceType::Queen),
        "r" | "R" => Ok(PieceType::Rook),
        "b" | "B" => Ok(PieceType::Bishop),
        "n" | "N" => Ok(PieceType::Knight),
        _ => Err("Invalid promotion piece".to_string()),
    }
}

fn personality_from_code(code: &str) -> Result<Personality, String> {
    let personality = match code {
        "aggressive" => Personality::Aggressive,
        "defensive" => Personality::Defensive,
        "chaotic" => Personality::Chaotic,
        _ => return Err("Invalid AI personality".to_string()),
    };

    Ok(personality)
}

fn coord_to_square(coord: &str) -> Result<Square, String> {
    let bytes = coord.as_bytes();
    if bytes.len() != 2 {
        return Err("Invalid square".to_string());
    }

    let file = match bytes[0].to_ascii_lowercase() {
        b'a'..=b'h' => (bytes[0].to_ascii_lowercase() - b'a') as usize,
        _ => return Err("Invalid file".to_string()),
    };

    let rank = match bytes[1] {
        b'1'..=b'8' => (bytes[1] - b'1') as usize,
        _ => return Err("Invalid rank".to_string()),
    };

    Square::from_index(file + rank * 8).ok_or_else(|| "Invalid square".to_string())
}

fn square_to_coord(square: Square) -> String {
    let file = match square.file() {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => unreachable!(),
    };
    let rank = char::from_digit((square.rank() + 1) as u32, 10).unwrap();
    format!("{file}{rank}")
}

fn json_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len() + 2);
    escaped.push('"');

    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(ch),
        }
    }

    escaped.push('"');
    escaped
}

fn option_json(value: Option<&str>) -> String {
    match value {
        Some(value) => json_string(value),
        None => "null".to_string(),
    }
}

fn api_error(stream: &mut TcpStream, message: &str) -> io::Result<()> {
    write_response(
        stream,
        "400 Bad Request",
        "text/plain; charset=utf-8",
        message.as_bytes(),
    )
}

fn write_response(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
) -> io::Result<()> {
    let headers = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(headers.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()
}
