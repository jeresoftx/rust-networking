use rust_networking::dns::{DnsRecord, DomainName, RecordType, Resolver, Zone};

fn main() {
    let mut zona = Zone::new();
    let nombre = DomainName::new("api.jeresoft.test").unwrap();

    zona.add_record(DnsRecord::a(nombre.clone(), "203.0.113.10", 300));

    let resolucion = Resolver::new(zona)
        .resolve(&nombre, RecordType::A, 0)
        .unwrap();

    println!("nombre canónico: {}", resolucion.canonical_name());
    println!("registro: {:?}", resolucion.records());
}
