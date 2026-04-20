// Integration tests for ChessQ API endpoints
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

const TEST_HOST: &str = "127.0.0.1:4173";

fn wait_for_server() {
    for _ in 0..10 {
        if TcpStream::connect(TEST_HOST).is_ok() {
            return;
        }
        thread::sleep(Duration::from_millis(100));
    }
    panic!("Server did not start in time");
}

fn make_request(path: &str) -> String {
    let mut stream = TcpStream::connect(TEST_HOST).expect("Failed to connect to server");
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, TEST_HOST);
    
    stream.write_all(request.as_bytes()).expect("Failed to write request");
    
    let mut response = String::new();
    stream.read_to_string(&mut response).expect("Failed to read response");
    
    response
}

fn extract_body(response: &str) -> &str {
    response.split("\r\n\r\n").nth(1).unwrap_or("")
}

#[test]
#[ignore] // Run with: cargo test --test integration_tests -- --ignored
fn test_api_new_game() {
    wait_for_server();
    
    let response = make_request("/api/new");
    let body = extract_body(&response);
    
    assert!(response.contains("200 OK"));
    assert!(body.contains("\"fen\""));
    assert!(body.contains("\"board\""));
    assert!(body.contains("\"turn\":\"w\""));
    assert!(body.contains("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
}

#[test]
#[ignore]
fn test_api_state_with_fen() {
    wait_for_server();
    
    let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR%20b%20KQkq%20e3%200%201";
    let response = make_request(&format!("/api/state?fen={}", fen));
    let body = extract_body(&response);
    
    assert!(response.contains("200 OK"));
    assert!(body.contains("\"turn\":\"b\""));
}

#[test]
#[ignore]
fn test_api_legal_moves() {
    wait_for_server();
    
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR%20w%20KQkq%20-%200%201";
    let response = make_request(&format!("/api/legal-moves?fen={}&from=e2", fen));
    let body = extract_body(&response);
    
    assert!(response.contains("200 OK"));
    assert!(body.contains("\"moves\""));
    assert!(body.contains("e3") || body.contains("e4"));
}

#[test]
#[ignore]
fn test_api_make_move() {
    wait_for_server();
    
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR%20w%20KQkq%20-%200%201";
    let response = make_request(&format!("/api/move?fen={}&from=e2&to=e4", fen));
    let body = extract_body(&response);
    
    assert!(response.contains("200 OK"));
    assert!(body.contains("\"fen\""));
    assert!(body.contains("\"turn\":\"b\""));
}

#[test]
#[ignore]
fn test_api_illegal_move() {
    wait_for_server();
    
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR%20w%20KQkq%20-%200%201";
    let response = make_request(&format!("/api/move?fen={}&from=e2&to=e5", fen));
    
    assert!(response.contains("400 Bad Request"));
}

#[test]
#[ignore]
fn test_api_ai_move() {
    wait_for_server();
    
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR%20w%20KQkq%20-%200%201";
    let response = make_request(&format!("/api/ai-move?fen={}&personality=aggressive&ms=500", fen));
    let body = extract_body(&response);
    
    assert!(response.contains("200 OK"));
    assert!(body.contains("\"fen\""));
    assert!(body.contains("\"evaluation\""));
    assert!(body.contains("\"depth\""));
}

#[test]
#[ignore]
fn test_api_ai_personalities() {
    wait_for_server();
    
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR%20w%20KQkq%20-%200%201";
    
    for personality in &["aggressive", "defensive", "chaotic"] {
        let response = make_request(&format!("/api/ai-move?fen={}&personality={}&ms=300", fen, personality));
        assert!(response.contains("200 OK"), "Failed for personality: {}", personality);
    }
}

#[test]
#[ignore]
fn test_api_promotion() {
    wait_for_server();
    
    // Position where white pawn can promote
    let fen = "8/P7/8/8/8/8/8/K6k%20w%20-%20-%200%201";
    let response = make_request(&format!("/api/move?fen={}&from=a7&to=a8&promotion=q", fen));
    let body = extract_body(&response);
    
    assert!(response.contains("200 OK"));
    assert!(body.contains("\"fen\""));
}

#[test]
#[ignore]
fn test_api_checkmate_detection() {
    wait_for_server();
    
    // Fool's mate position
    let fen = "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR%20w%20KQkq%20-%201%203";
    let response = make_request(&format!("/api/state?fen={}", fen));
    let body = extract_body(&response);
    
    assert!(response.contains("200 OK"));
    assert!(body.contains("checkmate") || body.contains("gameOver"));
}

#[test]
#[ignore]
fn test_static_file_serving() {
    wait_for_server();
    
    let response = make_request("/");
    assert!(response.contains("200 OK"));
    assert!(response.contains("text/html"));
}

#[test]
#[ignore]
fn test_static_js_file() {
    wait_for_server();
    
    let response = make_request("/play.js");
    assert!(response.contains("200 OK"));
    assert!(response.contains("text/javascript"));
}

#[test]
#[ignore]
fn test_static_css_file() {
    wait_for_server();
    
    let response = make_request("/play.css");
    assert!(response.contains("200 OK"));
    assert!(response.contains("text/css"));
}

#[test]
#[ignore]
fn test_404_not_found() {
    wait_for_server();
    
    let response = make_request("/nonexistent.html");
    assert!(response.contains("404 Not Found"));
}

#[test]
#[ignore]
fn test_path_traversal_protection() {
    wait_for_server();
    
    let response = make_request("/../etc/passwd");
    assert!(response.contains("403 Forbidden") || response.contains("404 Not Found"));
}

#[test]
#[ignore]
fn test_cors_headers() {
    wait_for_server();
    
    let response = make_request("/api/new");
    assert!(response.contains("Access-Control-Allow-Origin: *"));
}

// Helper function to run server in background for tests
// Note: In real usage, start server manually before running tests
// cargo run --release &
// cargo test --test integration_tests -- --ignored
