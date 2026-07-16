use rust_networking::smtp::{
    select_mx_by_priority, EmailHeaders, MailEnvelope, MxRecord, SmtpCommand, SmtpError,
    SmtpSession,
};

#[test]
fn accepts_basic_smtp_conversation() {
    let mut session = SmtpSession::new("mx.jeresoft.test");

    assert_eq!(
        session
            .apply(SmtpCommand::Helo("cliente.jeresoft.test".to_string()))
            .unwrap()
            .code(),
        250
    );
    assert_eq!(
        session
            .apply(SmtpCommand::MailFrom("profesor@jeresoft.test".to_string()))
            .unwrap()
            .code(),
        250
    );
    assert_eq!(
        session
            .apply(SmtpCommand::RcptTo("alumno@jeresoft.test".to_string()))
            .unwrap()
            .code(),
        250
    );
    assert_eq!(session.apply(SmtpCommand::Data).unwrap().code(), 354);
    assert_eq!(
        session
            .apply(SmtpCommand::MessageData(
                "Subject: Bienvenida\r\n\r\nHola".to_string()
            ))
            .unwrap()
            .code(),
        250
    );

    assert_eq!(session.server_name(), "mx.jeresoft.test");
    assert_eq!(session.accepted_messages().len(), 1);
    assert_eq!(
        session.accepted_messages()[0].envelope().sender(),
        "profesor@jeresoft.test"
    );
    assert_eq!(
        session.accepted_messages()[0].envelope().recipients(),
        &["alumno@jeresoft.test".to_string()]
    );
}

#[test]
fn separates_envelope_from_message_headers() {
    let envelope = MailEnvelope::new("bounce@jeresoft.test")
        .add_recipient("alumno@jeresoft.test")
        .unwrap();
    let headers = EmailHeaders::new()
        .with_header("From", "Profesor <profesor@jeresoft.test>")
        .with_header("To", "Alumno <alumno@jeresoft.test>")
        .with_header("Subject", "Bienvenida");

    assert_eq!(envelope.sender(), "bounce@jeresoft.test");
    assert_eq!(
        headers.get("from"),
        Some("Profesor <profesor@jeresoft.test>")
    );
    assert_ne!(envelope.sender(), headers.get("from").unwrap());
}

#[test]
fn selects_mx_record_by_lowest_priority() {
    let records = vec![
        MxRecord::new("mx-backup.jeresoft.test", 20),
        MxRecord::new("mx-primary.jeresoft.test", 10),
        MxRecord::new("mx-last.jeresoft.test", 50),
    ];

    let selected = select_mx_by_priority(&records).unwrap();

    assert_eq!(selected.exchange(), "mx-primary.jeresoft.test");
    assert_eq!(selected.priority(), 10);
}

#[test]
fn rejects_out_of_order_command() {
    let mut session = SmtpSession::new("mx.jeresoft.test");

    let error = session
        .apply(SmtpCommand::MailFrom("profesor@jeresoft.test".to_string()))
        .unwrap_err();

    assert_eq!(
        error,
        SmtpError::UnexpectedCommand {
            expected: "HELO".to_string(),
            actual: "MAIL FROM".to_string(),
        }
    );
}
