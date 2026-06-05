//! Build script: download VectorScan tarball, extract, and build static libs.
//! Reads version from workspace root .vectorscanrc.toml.

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
   println!("cargo:rerun-if-changed=build.rs");

   let rc_path = rc_path();
   println!("cargo:rerun-if-changed={}", rc_path.display());

   let vectorscan_version = read_vectorscan_version(&rc_path).unwrap_or_else(|e| {
      panic!(
         "auth-gateway-vectorscan: {} (required: .vectorscanrc.toml with [vendored] vectorscan_version)",
         e
      )
   });

   let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
   let tarball = out_dir.join("vectorscan.tar.gz");
   let src_dir = out_dir.join("vectorscan-src");

   // Reuse existing extract if present (e.g. incremental rebuilds)
   if !src_dir.join("CMakeLists.txt").exists() {
      download_vectorscan(&tarball, &vectorscan_version);
      extract_tarball(&tarball, &src_dir);

      // Clean up folders/files that link against pcre
      let tools_cmake = src_dir.join("tools").join("CMakeLists.txt");
      if tools_cmake.exists() {
         let _ = std::fs::remove_file(tools_cmake);
      }
      let unit_cmake = src_dir.join("unit").join("CMakeLists.txt");
      if unit_cmake.exists() {
         // Replace unit tests with an empty CMake list to bypass building them
         let _ = std::fs::write(unit_cmake, "");
      }
   }

   build_vectorscan(&src_dir, &out_dir);
   emit_links();
}

/// Path to .vectorscanrc.toml at workspace root (one level up from this crate).
fn rc_path() -> PathBuf {
   let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
   manifest.join(".vectorscanrc.toml")
}

/// Read vectorscan_version from .vectorscanrc.toml (TOML). Fails if file or key is missing.
fn read_vectorscan_version(rc_path: &PathBuf) -> Result<String, String> {
   let content = std::fs::read_to_string(rc_path).map_err(|e| {
      format!(
         ".vectorscanrc.toml not found or unreadable at {}: {}",
         rc_path.display(),
         e
      )
   })?;
   let table: toml::Table = toml::from_str(&content).map_err(|e| format!(".vectorscanrc.toml invalid TOML: {}", e))?;
   let vendored = table
      .get("vendored")
      .and_then(|v| v.as_table())
      .ok_or_else(|| ".vectorscanrc.toml must contain a [vendored] section".to_string())?;
   vendored
      .get("vectorscan_version")
      .and_then(|v| v.as_str())
      .map(|s| s.to_string())
      .ok_or_else(|| ".vectorscanrc.toml [vendored] must set vectorscan_version = \"…\" (e.g. \"5.4.11\")".to_string())
}

fn download_vectorscan(tarball: &Path, vectorscan_version: &str) {
   // VectorScan tags are "vectorscan/5.4.11" (slash, not "v" prefix)
   let tag = format!("vectorscan/{}", vectorscan_version);
   let url = format!(
      "https://github.com/VectorCamp/vectorscan/archive/refs/tags/{}.tar.gz",
      tag.replace('/', "%2F")
   );
   eprintln!("cargo:warning=Downloading VectorScan {}...", vectorscan_version);

   let status = Command::new("curl")
      .args(["-sfL", "-o", tarball.to_str().unwrap(), &url])
      .status()
      .expect("failed to run curl");

   if !status.success() {
      panic!("failed to download VectorScan from {} (is the network available?)", url);
   }
}

fn extract_tarball(tarball: &Path, dest: &PathBuf) {
   std::fs::create_dir_all(dest).expect("create vectorscan-src dir");

   let status = Command::new("tar")
      .args([
         "-xzf",
         tarball.to_str().unwrap(),
         "-C",
         dest.to_str().unwrap(),
         "--strip-components=1",
      ])
      .status()
      .expect("failed to run tar");

   if !status.success() {
      panic!("failed to extract VectorScan tarball");
   }
}

fn build_vectorscan(src_dir: &Path, out_dir: &Path) {
   let build_dir = out_dir.join("vectorscan-build");

   let dst = cmake::Config::new(src_dir)
      .out_dir(&build_dir)
      .define("BUILD_SHARED_LIBS", "OFF")
      .define("BUILD_STATIC_LIBS", "ON")
      .define("BUILD_CHIMERA", "OFF")
      .define("BUILD_EXAMPLES", "OFF")
      .define("BUILD_BENCHMARKS", "OFF")
      .build();

   // Tell Cargo where to find the built libs (cmake installs into prefix/lib)
   let lib_dir = dst.join("lib");
   println!("cargo:rustc-link-search=native={}", lib_dir.display());
   println!("cargo:rerun-if-changed={}", src_dir.join("CMakeLists.txt").display());
}

fn emit_links() {
   println!("cargo:rustc-link-lib=static=hs");
   println!("cargo:rustc-link-lib=static=hs_runtime");

   if cfg!(target_os = "macos") {
      println!("cargo:rustc-link-lib=c++");
   } else {
      println!("cargo:rustc-link-lib=stdc++");
   }
}
