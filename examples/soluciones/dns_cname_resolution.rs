use rust_networking::dns::{DnsRecord, DomainName, RecordType, Resolver, Zone};

fn main() {
    let mut zona = Zone::new();
    let api = DomainName::new("api.jeresoft.test").unwrap();
    let origen = DomainName::new("origin.jeresoft.test").unwrap();

    zona.add_record(DnsRecord::cname(api.clone(), origen.clone(), 300));
    zona.add_record(DnsRecord::a(origen.clone(), "203.0.113.20", 300));

    let resolucion = Resolver::new(zona).resolve(&api, RecordType::A, 0).unwrap();

    assert_eq!(resolucion.canonical_name(), &origen);
}
