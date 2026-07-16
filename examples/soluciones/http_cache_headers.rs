use rust_networking::http::HeaderMap;

fn main() {
    let encabezados = HeaderMap::new()
        .with_cache_control("max-age=60")
        .with_etag("\"lesson-1\"");

    assert_eq!(encabezados.cache_control(), Some("max-age=60"));
    assert_eq!(encabezados.etag(), Some("\"lesson-1\""));
    println!("cache-control: {}", encabezados.cache_control().unwrap());
    println!("etag: {}", encabezados.etag().unwrap());
}
