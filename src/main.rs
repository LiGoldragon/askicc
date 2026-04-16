/// askicc — the bootstrap compiler.
///
/// Reads .synth dialect files → populates aski-core domain-data-tree
/// → serializes as rkyv.
///
/// Usage: askicc <synth-dir> <output-file>

use std::fs;
use std::path::Path;

use askicc::synth_lex;
use askicc::synth_parse::Dialect;

struct Askicc {
    dialects: Vec<Dialect>,
}

impl Askicc {
    fn load(synth_dir: &Path) -> Self {
        let dialects = Self::load_dialects(synth_dir);
        Askicc { dialects }
    }

    fn load_dialects(dir: &Path) -> Vec<Dialect> {
        let mut files: Vec<_> = fs::read_dir(dir)
            .unwrap_or_else(|e| panic!("failed to read {}: {}", dir.display(), e))
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|x| x == "synth").unwrap_or(false))
            .collect();
        files.sort_by_key(|e| e.path());

        files.iter().map(|entry| {
            let path = entry.path();
            let name = path.file_stem().unwrap().to_string_lossy().to_string();
            let source = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e));
            let tokens = synth_lex::synth_lex(&source)
                .unwrap_or_else(|e| panic!("synth lex error in {}: {}", path.display(), e));
            let dialect = Dialect::parse(&name, &tokens)
                .unwrap_or_else(|e| panic!("synth parse error in {}: {}", path.display(), e));
            eprintln!("askicc: synth {} → {} rules", name, dialect.rules.len());
            dialect
        }).collect()
    }

    fn serialize(&self, out_path: &Path) {
        // TODO: serialize self.dialects as rkyv using aski-core types
        // For now: write dialect count as a placeholder
        let summary = format!("{} dialects loaded\n", self.dialects.len());
        fs::write(out_path, summary.as_bytes())
            .unwrap_or_else(|e| panic!("failed to write {}: {}", out_path.display(), e));
        eprintln!("askicc: wrote {} ({} dialects)", out_path.display(), self.dialects.len());
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (synth_dir, out_path) = match args.len() {
        3 => (Path::new(&args[1]), Path::new(&args[2])),
        _ => {
            // Default: read from source/, write to generated/dialects.rkyv
            (Path::new("source"), Path::new("generated/dialects.rkyv"))
        }
    };

    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent).expect("failed to create output directory");
    }

    let compiler = Askicc::load(synth_dir);
    compiler.serialize(out_path);
}
