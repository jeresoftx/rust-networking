use std::hint::black_box;
use std::time::Instant;

use rust_networking::dns::{DnsRecord, DomainName, RecordType, Resolver, Zone};

fn construir_zona() -> (Zone, DomainName, DomainName) {
    let mut zona = Zone::new();
    let api = DomainName::new("api.jeresoft.test").unwrap();
    let alias = DomainName::new("alias.jeresoft.test").unwrap();
    let origen = DomainName::new("origin.jeresoft.test").unwrap();

    zona.add_record(DnsRecord::a(api.clone(), "203.0.113.10", 300));
    zona.add_record(DnsRecord::cname(alias.clone(), origen.clone(), 300));
    zona.add_record(DnsRecord::a(origen, "203.0.113.20", 300));

    (zona, api, alias)
}

fn resolver_cache_fria(iteraciones: usize) {
    for instante in 0..iteraciones {
        let (zona, api, _) = construir_zona();
        let mut resolvedor = Resolver::new(zona);
        let resolucion = resolvedor
            .resolve(&api, RecordType::A, instante as u64)
            .unwrap();
        black_box(resolucion);
    }
}

fn resolver_cache_caliente(iteraciones: usize) {
    let (zona, api, _) = construir_zona();
    let mut resolvedor = Resolver::new(zona);
    black_box(resolvedor.resolve(&api, RecordType::A, 0).unwrap());

    for instante in 1..iteraciones {
        let resolucion = resolvedor
            .resolve(&api, RecordType::A, instante as u64)
            .unwrap();
        black_box(resolucion);
    }
}

fn resolver_cname(iteraciones: usize) {
    let (zona, _, alias) = construir_zona();
    let mut resolvedor = Resolver::new(zona);

    for instante in 0..iteraciones {
        let resolucion = resolvedor
            .resolve(&alias, RecordType::A, instante as u64)
            .unwrap();
        black_box(resolucion);
    }
}

fn main() {
    let iteraciones = 20_000usize;

    let start = Instant::now();
    resolver_cache_fria(iteraciones);
    let fria_elapsed = start.elapsed();

    let start = Instant::now();
    resolver_cache_caliente(iteraciones);
    let caliente_elapsed = start.elapsed();

    let start = Instant::now();
    resolver_cname(iteraciones);
    let cname_elapsed = start.elapsed();

    println!("benchmark de dns (manual, std::time::Instant)");
    println!("iteraciones: {iteraciones}");
    println!("caché fría: {fria_elapsed:?}");
    println!("caché caliente: {caliente_elapsed:?}");
    println!("resolución con CNAME: {cname_elapsed:?}");
}
