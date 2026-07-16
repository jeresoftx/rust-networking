use rust_networking::dns::{DnsError, DnsRecord, DomainName, RecordType, Resolver, Zone};

#[test]
fn zone_resolves_a_record_for_domain_name() {
    let mut zone = Zone::new();
    let name = DomainName::new("api.jeresoft.test").unwrap();

    zone.add_record(DnsRecord::a(name.clone(), "203.0.113.10", 300));

    let resolution = Resolver::new(zone)
        .resolve(&name, RecordType::A, 0)
        .unwrap();

    assert_eq!(resolution.canonical_name(), &name);
    assert_eq!(
        resolution.records(),
        &[DnsRecord::a(name, "203.0.113.10", 300)]
    );
    assert!(!resolution.from_cache());
}

#[test]
fn resolver_follows_cname_chain_until_limit() {
    let mut zone = Zone::new();
    let api = DomainName::new("api.jeresoft.test").unwrap();
    let edge = DomainName::new("edge.jeresoft.test").unwrap();
    let origin = DomainName::new("origin.jeresoft.test").unwrap();

    zone.add_record(DnsRecord::cname(api.clone(), edge.clone(), 300));
    zone.add_record(DnsRecord::cname(edge, origin.clone(), 300));
    zone.add_record(DnsRecord::a(origin.clone(), "203.0.113.20", 300));

    let resolution = Resolver::new(zone).resolve(&api, RecordType::A, 0).unwrap();

    assert_eq!(resolution.canonical_name(), &origin);
    assert_eq!(
        resolution.records(),
        &[DnsRecord::a(origin, "203.0.113.20", 300)]
    );
}

#[test]
fn resolver_uses_cache_until_ttl_expires() {
    let mut zone = Zone::new();
    let name = DomainName::new("cdn.jeresoft.test").unwrap();

    zone.add_record(DnsRecord::a(name.clone(), "203.0.113.30", 10));

    let mut resolver = Resolver::new(zone);

    assert!(!resolver
        .resolve(&name, RecordType::A, 100)
        .unwrap()
        .from_cache());
    assert!(resolver
        .resolve(&name, RecordType::A, 109)
        .unwrap()
        .from_cache());
    assert!(!resolver
        .resolve(&name, RecordType::A, 111)
        .unwrap()
        .from_cache());
}

#[test]
fn resolver_returns_nxdomain_for_unknown_name() {
    let zone = Zone::new();
    let name = DomainName::new("missing.jeresoft.test").unwrap();

    let error = Resolver::new(zone)
        .resolve(&name, RecordType::A, 0)
        .unwrap_err();

    assert_eq!(error, DnsError::NxDomain { name });
}
