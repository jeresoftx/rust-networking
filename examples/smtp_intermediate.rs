use rust_networking::smtp::{EmailHeaders, MailEnvelope};

fn main() {
    let sobre = MailEnvelope::new("bounce@jeresoft.test")
        .add_recipient("alumno@jeresoft.test")
        .unwrap();
    let encabezados = EmailHeaders::new()
        .with_header("From", "Profesor <profesor@jeresoft.test>")
        .with_header("To", "Alumno <alumno@jeresoft.test>")
        .with_header("Subject", "Bienvenida");

    println!("remitente del sobre: {}", sobre.sender());
    println!("encabezado From: {}", encabezados.get("From").unwrap());
    println!("asunto: {}", encabezados.get("Subject").unwrap());
}
