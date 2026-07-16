use rust_networking::dns::{DnsRecord, DomainName, RecordType, Resolver, Zone};

fn main() {
    let mut zona = Zone::new();
    let nombre = DomainName::new("cdn.jeresoft.test").unwrap();

    zona.add_record(DnsRecord::a(nombre.clone(), "203.0.113.30", 10));

    let mut resolvedor = Resolver::new(zona);

    let primera = resolvedor.resolve(&nombre, RecordType::A, 100).unwrap();
    let segunda = resolvedor.resolve(&nombre, RecordType::A, 109).unwrap();
    let tercera = resolvedor.resolve(&nombre, RecordType::A, 111).unwrap();

    println!("primera desde caché: {}", primera.from_cache());
    println!("segunda desde caché: {}", segunda.from_cache());
    println!("tercera desde caché: {}", tercera.from_cache());
}
