use rust_networking::quic::{ConnectionMigration, QuicConnectionId, QuicHandshake};

fn main() {
    let connection = QuicConnectionId::new("cid-clase-en-vivo").unwrap();
    let handshake = QuicHandshake::negotiate(connection.clone(), "TLS 1.3", "h3").unwrap();
    let migration = ConnectionMigration::new(
        connection,
        "wifi:casa:4433",
        "lte:movil:4433",
        "el estudiante salió de casa durante una clase en vivo",
    );

    println!("segura: {}", handshake.is_secure());
    println!(
        "misma conexión tras migrar: {}",
        migration.preserves_connection_identity()
    );
}
