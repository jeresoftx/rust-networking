use rust_networking::dns::{DnsRecord, DomainName, RecordType, Resolver, Zone};

fn main() {
    let mut zona = Zone::new();
    let nombre = DomainName::new("cdn.jeresoft.test").unwrap();

    zona.add_record(DnsRecord::a(nombre.clone(), "203.0.113.30", 10));

    let mut resolvedor = Resolver::new(zona);

    assert!(!resolvedor
        .resolve(&nombre, RecordType::A, 100)
        .unwrap()
        .from_cache());
    assert!(resolvedor
        .resolve(&nombre, RecordType::A, 109)
        .unwrap()
        .from_cache());
    assert!(!resolvedor
        .resolve(&nombre, RecordType::A, 111)
        .unwrap()
        .from_cache());
}
