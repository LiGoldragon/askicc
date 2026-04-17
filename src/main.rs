/// askicc — bootstrap compiler.
///
/// Reads .synth dialect files → rkyv domain-data-tree.
///
/// Usage: askicc <synth-dir> <output-file>

use std::fs;
use std::path::Path;

use synth_core::{DialectTree, Dialect};
use askicc::synth_lex::SynthLexer;
use askicc::synth_parse::SynthParser;

struct Askicc {
    dialects: Vec<Dialect>,
}

impl Askicc {
    fn load(synth_dir: &Path) -> Result<Self, String> {
        let mut files: Vec<_> = fs::read_dir(synth_dir)
            .map_err(|e| format!("failed to read {}: {}", synth_dir.display(), e))?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|x| x == "synth").unwrap_or(false))
            .collect();
        files.sort_by_key(|e| e.path());

        let mut dialects = Vec::new();
        for entry in &files {
            let path = entry.path();
            let name = path.file_stem().unwrap().to_string_lossy().to_string();
            let source = fs::read_to_string(&path)
                .map_err(|e| format!("failed to read {}: {}", path.display(), e))?;

            let tokens = SynthLexer::new(&source).lex()
                .map_err(|e| format!("lex error in {}: {}", path.display(), e))?;

            let kind = SynthLexer::resolve_filename(&name)
                .map_err(|e| format!("unknown dialect {}: {}", path.display(), e))?;

            let dialect = SynthParser::new(&tokens).parse(kind)
                .map_err(|e| format!("parse error in {}: {}", path.display(), e))?;

            eprintln!("askicc: {} → {} rules", name, dialect.rules.len());
            dialects.push(dialect);
        }

        Ok(Askicc { dialects })
    }

    fn serialize(&self, out_path: &Path) -> Result<(), String> {
        let tree = DialectTree { dialects: self.dialects.clone() };
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&tree)
            .map_err(|e| format!("rkyv serialization failed: {}", e))?;

        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("failed to create dir: {}", e))?;
        }

        fs::write(out_path, bytes.as_ref())
            .map_err(|e| format!("failed to write {}: {}", out_path.display(), e))?;

        eprintln!("askicc: wrote {} ({} bytes, {} dialects)",
            out_path.display(), bytes.len(), self.dialects.len());
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (synth_dir, out_path) = match args.len() {
        3 => (Path::new(&args[1]), Path::new(&args[2])),
        _ => (Path::new("source"), Path::new("generated/dialects.rkyv")),
    };

    let compiler = Askicc::load(synth_dir)
        .unwrap_or_else(|e| { eprintln!("askicc: {}", e); std::process::exit(1); });

    compiler.serialize(out_path)
        .unwrap_or_else(|e| { eprintln!("askicc: {}", e); std::process::exit(1); });
}
