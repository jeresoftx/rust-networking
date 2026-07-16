use rust_networking::https::{HstsPolicy, HttpsPolicy};

fn main() {
    let politica = HttpsPolicy::new(
        HstsPolicy::new()
            .include_host("api.jeresoft.test")
            .include_subdomains(),
    );

    assert!(politica.should_force_https("api.jeresoft.test"));
    assert!(politica.should_force_https("v1.api.jeresoft.test"));
    println!("HSTS cubre host y subdominios");
}
