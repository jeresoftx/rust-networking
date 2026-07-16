use rust_networking::quic::{ConnectionMigration, QuicConnectionId};

fn main() {
    let connection = QuicConnectionId::new("cid-solucion-migracion").unwrap();
    let migration = ConnectionMigration::new(connection, "wifi", "lte", "cambio de red");

    assert_eq!(migration.previous_path(), "wifi");
    assert_eq!(migration.current_path(), "lte");
    assert!(migration.preserves_connection_identity());
}
