use rust_networking::smtp::{SmtpCommand, SmtpSession};

fn main() {
    let mut sesion = SmtpSession::new("mx.jeresoft.test");

    sesion
        .apply(SmtpCommand::Helo("cliente.jeresoft.test".to_string()))
        .unwrap();
    sesion
        .apply(SmtpCommand::MailFrom("profesor@jeresoft.test".to_string()))
        .unwrap();
    sesion
        .apply(SmtpCommand::RcptTo("alumno@jeresoft.test".to_string()))
        .unwrap();
    sesion.apply(SmtpCommand::Data).unwrap();
    sesion
        .apply(SmtpCommand::MessageData(
            "Subject: Hola\r\n\r\nBienvenido".to_string(),
        ))
        .unwrap();

    assert_eq!(sesion.accepted_messages().len(), 1);
    println!("conversación aceptada");
}
