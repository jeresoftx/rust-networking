use rust_networking::smtp::{select_mx_by_priority, MxRecord, SmtpCommand, SmtpError, SmtpSession};

fn enviar_notificacion() -> Result<usize, SmtpError> {
    let registros = vec![
        MxRecord::new("mx2.jeresoft.test", 20),
        MxRecord::new("mx1.jeresoft.test", 10),
    ];
    let mx = select_mx_by_priority(&registros).unwrap();
    let mut sesion = SmtpSession::new(mx.exchange());

    sesion.apply(SmtpCommand::Helo("academy-web.jeresoft.test".to_string()))?;
    sesion.apply(SmtpCommand::MailFrom("bounce@jeresoft.test".to_string()))?;
    sesion.apply(SmtpCommand::RcptTo("alumno@jeresoft.test".to_string()))?;
    sesion.apply(SmtpCommand::Data)?;
    sesion.apply(SmtpCommand::MessageData(
        "From: Jeresoft Academy <no-reply@jeresoft.test>\r\n\
         To: Alumno <alumno@jeresoft.test>\r\n\
         Subject: Ruta de estudio\r\n\r\n\
         Ya tienes una nueva lección disponible."
            .to_string(),
    ))?;

    Ok(sesion.accepted_messages().len())
}

fn main() {
    let aceptados = enviar_notificacion().unwrap();
    println!("mensajes en cola saliente: {aceptados}");
}
