/// askicc — bootstrap compiler.
///
/// Reads .synth files from source/<surface>/ directories and
/// produces a single rkyv `dsls.rkyv` containing every dialect
/// from every DSL (core, aski, synth, exec) flattened into one
/// DialectTree. Each Dialect carries its SurfaceKind so askic
/// can dispatch by (Surface, DialectKind).
///
/// Terminology: a DSL (one of four surfaces) is made of many
/// dialects (Body, Statement, Expr, …). One .synth file = one
/// dialect. This rkyv bundles all four DSLs.
///
/// Usage: askicc <source-root> <output-file>

use std::fs;
use std::path::{Path, PathBuf};

use synth_core::{DialectTree, Dialect, SurfaceKind};
use askicc::synth_lex::SynthLexer;
use askicc::synth_parse::SynthParser;

struct Askicc {
    dialects: Vec<Dialect>,
}

impl Askicc {
    fn load(source_root: &Path) -> Result<Self, String> {
        let mut surface_dirs: Vec<_> = fs::read_dir(source_root)
            .map_err(|e| format!("failed to read {}: {}", source_root.display(), e))?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .collect();
        surface_dirs.sort_by_key(|e| e.path());

        let mut dialects = Vec::new();
        for surface_entry in &surface_dirs {
            let surface_path = surface_entry.path();
            let surface_name = surface_path.file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| format!("invalid surface dir: {}", surface_path.display()))?
                .to_string();

            let surface_kind = SynthLexer::resolve_surface_kind(&surface_name)
                .map_err(|e| format!("surface {}: {}", surface_path.display(), e))?;

            let mut files: Vec<PathBuf> = fs::read_dir(&surface_path)
                .map_err(|e| format!("failed to read {}: {}", surface_path.display(), e))?
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().map(|x| x == "synth").unwrap_or(false))
                .collect();
            files.sort();

            for path in &files {
                let name = path.file_stem().unwrap().to_string_lossy().to_string();
                let source = fs::read_to_string(path)
                    .map_err(|e| format!("failed to read {}: {}", path.display(), e))?;

                let tokens = SynthLexer::new(&source).lex()
                    .map_err(|e| format!("lex error in {}: {}", path.display(), e))?;

                let kind = SynthLexer::resolve_filename(&name)
                    .map_err(|e| format!("unknown dialect {}: {}", path.display(), e))?;

                let dialect = SynthParser::new(&tokens).parse(surface_kind, kind)
                    .map_err(|e| format!("parse error in {}: {}", path.display(), e))?;

                eprintln!("askicc: {}/{} → {} rules", surface_name, name, dialect.rules.len());
                dialects.push(dialect);
            }
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

        let surfaces: Vec<_> = [SurfaceKind::Core, SurfaceKind::Aski, SurfaceKind::Synth, SurfaceKind::Exec, SurfaceKind::Ffi]
            .iter()
            .map(|s| (s, self.dialects.iter().filter(|d| &d.surface == s).count()))
            .filter(|(_, n)| *n > 0)
            .map(|(s, n)| format!("{:?}={}", s, n))
            .collect();

        eprintln!("askicc: wrote {} ({} bytes, {} dialects across [{}])",
            out_path.display(), bytes.len(), self.dialects.len(), surfaces.join(", "));
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (source_root, out_path) = match args.len() {
        3 => (PathBuf::from(&args[1]), PathBuf::from(&args[2])),
        _ => (PathBuf::from("source"), PathBuf::from("generated/dsls.rkyv")),
    };

    let compiler = Askicc::load(&source_root)
        .unwrap_or_else(|e| { eprintln!("askicc: {}", e); std::process::exit(1); });

    compiler.serialize(&out_path)
        .unwrap_or_else(|e| { eprintln!("askicc: {}", e); std::process::exit(1); });
}
