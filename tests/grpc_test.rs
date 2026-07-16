use rust_networking::grpc::{
    GrpcError, GrpcMessage, GrpcMethod, GrpcService, GrpcStatus, StreamMode,
};

#[test]
fn registers_service_method_contract() {
    let method = GrpcMethod::new("GetLesson", 1, StreamMode::Unary);
    let service = GrpcService::new("academy.LessonService", 1)
        .add_method(method)
        .unwrap();

    let registered = service.method("GetLesson").unwrap();

    assert_eq!(service.name(), "academy.LessonService");
    assert_eq!(service.version(), 1);
    assert_eq!(registered.name(), "GetLesson");
    assert_eq!(registered.mode(), StreamMode::Unary);
}

#[test]
fn maps_grpc_status_codes() {
    assert_eq!(GrpcStatus::ok().code(), 0);
    assert_eq!(GrpcStatus::not_found("lección no encontrada").code(), 5);
    assert_eq!(
        GrpcStatus::unavailable("servicio no disponible").name(),
        "UNAVAILABLE"
    );
}

#[test]
fn represents_all_stream_modes() {
    assert!(StreamMode::Unary.is_unary());
    assert!(StreamMode::ServerStreaming.has_server_stream());
    assert!(StreamMode::ClientStreaming.has_client_stream());
    assert!(StreamMode::BidirectionalStreaming.has_client_stream());
    assert!(StreamMode::BidirectionalStreaming.has_server_stream());
}

#[test]
fn rejects_incompatible_contract_version() {
    let method = GrpcMethod::new("GetLesson", 2, StreamMode::Unary);
    let service = GrpcService::new("academy.LessonService", 1)
        .add_method(method)
        .unwrap();

    let error = service.ensure_compatible("GetLesson", 1).unwrap_err();

    assert_eq!(
        error,
        GrpcError::IncompatibleVersion {
            method: "GetLesson".to_string(),
            expected: 1,
            actual: 2,
        }
    );
}

#[test]
fn message_keeps_type_name_and_payload() {
    let message = GrpcMessage::new("GetLessonRequest", br#"{"id":42}"#.to_vec());

    assert_eq!(message.type_name(), "GetLessonRequest");
    assert_eq!(message.payload(), br#"{"id":42}"#);
}
