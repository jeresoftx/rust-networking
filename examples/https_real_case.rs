use rust_networking::http::{HttpMethod, HttpRequest, HttpResponse, StatusCode};
use rust_networking::https::{HttpsRequest, SecureTransport};
use rust_networking::tls::{
    Certificate, CertificateChain, CipherSuite, TlsClientHello, TlsVersion,
};

fn conectar_api(nombre_certificado: &str) -> Result<HttpResponse, String> {
    let solicitud_http =
        HttpRequest::parse("GET /academy/status HTTP/1.1\r\nHost: api.jeresoft.test\r\n\r\n")
            .unwrap();
    let solicitud_https = HttpsRequest::new("api.jeresoft.test", solicitud_http);
    let cliente = TlsClientHello::new(
        "api.jeresoft.test",
        vec![TlsVersion::V1_3],
        vec![CipherSuite::TlsAes128GcmSha256],
    );
    let cadena = CertificateChain::new(vec![
        Certificate::new(nombre_certificado, "Jeresoft Academy CA"),
        Certificate::new("Jeresoft Academy CA", "Jeresoft Academy Root"),
    ]);

    let transporte = SecureTransport::connect(
        solicitud_https,
        cliente,
        cadena,
        vec![TlsVersion::V1_3],
        vec![CipherSuite::TlsAes128GcmSha256],
    )
    .map_err(|error| format!("conexión HTTPS rechazada: {error:?}"))?;

    if transporte.request().method() == HttpMethod::Get
        && transporte.request().path() == "/academy/status"
    {
        Ok(HttpResponse::new(StatusCode::Ok)
            .with_header("Content-Type", "text/plain")
            .with_body("servicio seguro disponible".as_bytes().to_vec()))
    } else {
        Ok(HttpResponse::new(StatusCode::NotFound))
    }
}

fn main() {
    let respuesta = conectar_api("api.jeresoft.test").unwrap();
    println!(
        "respuesta: {} {}",
        respuesta.status().code(),
        respuesta.reason_phrase()
    );
    println!("cuerpo: {}", String::from_utf8_lossy(respuesta.body()));

    let error = conectar_api("otro.jeresoft.test").unwrap_err();
    println!("{error}");
}
