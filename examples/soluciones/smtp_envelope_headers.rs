use rust_networking::smtp::{EmailHeaders, MailEnvelope};

fn main() {
    let sobre = MailEnvelope::new("bounce@jeresoft.test")
        .add_recipient("alumno@jeresoft.test")
        .unwrap();
    let encabezados = EmailHeaders::new()
        .with_header("From", "Profesor <profesor@jeresoft.test>")
        .with_header("To", "Alumno <alumno@jeresoft.test>")
        .with_header("Subject", "Bienvenida");

    assert_eq!(sobre.sender(), "bounce@jeresoft.test");
    assert_eq!(
        encabezados.get("From"),
        Some("Profesor <profesor@jeresoft.test>")
    );
    assert_ne!(sobre.sender(), encabezados.get("From").unwrap());
    println!("sobre y encabezados separados");
}
