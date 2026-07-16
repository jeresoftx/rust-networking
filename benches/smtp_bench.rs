use std::hint::black_box;
use std::time::Instant;

use rust_networking::smtp::{select_mx_by_priority, MxRecord, SmtpCommand, SmtpError, SmtpSession};

fn conversacion_valida(iteraciones: usize) {
    for _ in 0..iteraciones {
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
        black_box(sesion);
    }
}

fn rechazar_fuera_de_orden(iteraciones: usize) {
    for _ in 0..iteraciones {
        let mut sesion = SmtpSession::new("mx.jeresoft.test");
        let error = sesion
            .apply(SmtpCommand::MailFrom("profesor@jeresoft.test".to_string()))
            .unwrap_err();
        black_box(error);
    }
}

fn seleccionar_mx(iteraciones: usize) {
    let registros = vec![
        MxRecord::new("mx-backup.jeresoft.test", 20),
        MxRecord::new("mx-primary.jeresoft.test", 10),
        MxRecord::new("mx-last.jeresoft.test", 50),
    ];

    for _ in 0..iteraciones {
        let seleccionado = select_mx_by_priority(&registros).unwrap();
        black_box(seleccionado);
    }
}

fn propagar_error(error: SmtpError) {
    black_box(error);
}

fn main() {
    let iteraciones = 20_000usize;

    let start = Instant::now();
    conversacion_valida(iteraciones);
    let conversacion_elapsed = start.elapsed();

    let start = Instant::now();
    rechazar_fuera_de_orden(iteraciones);
    let rechazo_elapsed = start.elapsed();

    let start = Instant::now();
    seleccionar_mx(iteraciones);
    let mx_elapsed = start.elapsed();

    propagar_error(SmtpError::EmptyRecipient);

    println!("benchmark de smtp (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("conversación válida: {conversacion_elapsed:?}");
    println!("rechazo fuera de orden: {rechazo_elapsed:?}");
    println!("selección de MX: {mx_elapsed:?}");
}
