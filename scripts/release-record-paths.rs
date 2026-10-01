//! The release boundary shares the product's config and glob interpretation.
use kotowari_core::{change_records, config::Config};

fn main() {
    let result = (|| {
        let yaml = std::fs::read_to_string(".kotowari/config.yaml").map_err(|e| e.to_string())?;
        let config = Config::parse(&yaml).map_err(|e| format!("{e:?}"))?;
        let changes = config.changes.ok_or("changes configuration is required")?;
        let records = change_records::glob(&changes.records);
        for path in std::env::args().skip(1) {
            if !change_records::normalized_relative(&path) || !records.is_match(&path) {
                return Err(format!("not a configured record path: {path}"));
            }
        }
        Ok::<(), String>(())
    })();
    if let Err(error) = result {
        eprintln!("release record boundary: {error}");
        std::process::exit(1);
    }
}
