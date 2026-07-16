use rust_networking::smtp::{SmtpCommand, SmtpSession};

fn main() {
    let mut sesion = SmtpSession::new("mx.jeresoft.test");

    for comando in [
        SmtpCommand::Helo("cliente.jeresoft.test".to_string()),
        SmtpCommand::MailFrom("profesor@jeresoft.test".to_string()),
        SmtpCommand::RcptTo("alumno@jeresoft.test".to_string()),
        SmtpCommand::Data,
        SmtpCommand::MessageData("Subject: Hola\r\n\r\nBienvenido".to_string()),
    ] {
        let respuesta = sesion.apply(comando).unwrap();
        println!("{} {}", respuesta.code(), respuesta.message());
    }

    println!("mensajes aceptados: {}", sesion.accepted_messages().len());
}
