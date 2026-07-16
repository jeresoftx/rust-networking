use rust_networking::http::HttpRequest;
use rust_networking::websocket::{
    CloseCode, Opcode, WebSocketError, WebSocketFrame, WebSocketState, WebSocketUpgrade,
};

fn upgrade_request() -> HttpRequest {
    HttpRequest::parse(
        "GET /notificaciones HTTP/1.1\r\n\
         Host: academy.jeresoft.test\r\n\
         Connection: Upgrade\r\n\
         Upgrade: websocket\r\n\
         Sec-WebSocket-Key: clave-educativa\r\n\r\n",
    )
    .unwrap()
}

#[test]
fn accepts_http_upgrade_request() {
    let connection = WebSocketUpgrade::new(upgrade_request()).accept().unwrap();

    assert_eq!(connection.state(), WebSocketState::Open);
    assert_eq!(connection.path(), "/notificaciones");
}

#[test]
fn text_frame_stores_opcode_and_payload() {
    let frame = WebSocketFrame::text("hola");

    assert_eq!(frame.opcode(), Opcode::Text);
    assert_eq!(frame.payload(), b"hola");
    assert!(!frame.is_control());
}

#[test]
fn ping_frame_generates_pong_with_same_payload() {
    let ping = WebSocketFrame::ping(b"latido".to_vec());
    let pong = ping.respond_to_ping().unwrap();

    assert_eq!(pong.opcode(), Opcode::Pong);
    assert_eq!(pong.payload(), b"latido");
    assert!(pong.is_control());
}

#[test]
fn close_frame_moves_connection_to_closed() {
    let mut connection = WebSocketUpgrade::new(upgrade_request()).accept().unwrap();
    let frame = WebSocketFrame::close(CloseCode::NormalClosure);

    connection.apply_frame(frame).unwrap();

    assert_eq!(connection.state(), WebSocketState::Closed);
}

#[test]
fn rejects_missing_upgrade_header() {
    let request = HttpRequest::parse(
        "GET /notificaciones HTTP/1.1\r\n\
         Host: academy.jeresoft.test\r\n\
         Connection: Upgrade\r\n\r\n",
    )
    .unwrap();
    let error = WebSocketUpgrade::new(request).accept().unwrap_err();

    assert_eq!(
        error,
        WebSocketError::InvalidUpgradeHeader {
            name: "Upgrade".to_string(),
        }
    );
}
