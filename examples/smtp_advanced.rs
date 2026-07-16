use rust_networking::smtp::{select_mx_by_priority, MxRecord};

fn main() {
    let registros = vec![
        MxRecord::new("mx-backup.jeresoft.test", 20),
        MxRecord::new("mx-primary.jeresoft.test", 10),
        MxRecord::new("mx-last.jeresoft.test", 50),
    ];

    let elegido = select_mx_by_priority(&registros).unwrap();

    println!(
        "servidor MX elegido: {} prioridad {}",
        elegido.exchange(),
        elegido.priority()
    );
}
