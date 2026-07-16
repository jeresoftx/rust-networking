use rust_networking::https::{HstsPolicy, HttpsPolicy};

fn main() {
    let politica = HttpsPolicy::new(
        HstsPolicy::new()
            .include_host("api.jeresoft.test")
            .include_subdomains(),
    );

    for host in [
        "api.jeresoft.test",
        "v1.api.jeresoft.test",
        "otro.jeresoft.test",
    ] {
        println!(
            "{host}: forzar HTTPS = {}",
            politica.should_force_https(host)
        );
    }
}
