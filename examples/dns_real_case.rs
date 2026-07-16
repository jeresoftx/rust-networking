use rust_networking::dns::{DnsRecord, DomainName, RecordType, Resolver, Zone};

fn main() {
    let mut zona = Zone::new();
    let api = DomainName::new("api.reservas.test").unwrap();
    let borde = DomainName::new("edge.reservas.test").unwrap();
    let origen = DomainName::new("origin.reservas.test").unwrap();

    zona.add_record(DnsRecord::cname(api.clone(), borde.clone(), 60));
    zona.add_record(DnsRecord::cname(borde, origen.clone(), 60));
    zona.add_record(DnsRecord::a(origen.clone(), "203.0.113.44", 60));
    zona.add_record(DnsRecord::txt(
        DomainName::new("reservas.test").unwrap(),
        "owner=jeresoft-academy",
        300,
    ));

    let mut resolvedor = Resolver::new(zona);
    let resolucion = resolvedor.resolve(&api, RecordType::A, 1_000).unwrap();

    println!("api solicitada: {api}");
    println!("nombre canónico: {}", resolucion.canonical_name());
    println!("dirección final: {}", resolucion.records()[0].value());
}
