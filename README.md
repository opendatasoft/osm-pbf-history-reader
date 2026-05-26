# OpenStreetMap PBF history parser

This Rust program:
- parses OSM [history files from Geofabrik](https://osm-internal.download.geofabrik.de)
- then creates a postgres `{schema}.{country_code}_history` table with the following fields:
  - id (negative IDs are for relations)
  - timestamps
  - changesets
  - first_timestamps
  - last_timestamps
  - users_number
  - versions_number

All countries share a single schema `huwise_osm`, with tables prefixed by country code (e.g. `fr_history`, `au_history`).

---

## Usage

### Basic usage (France, default)
```bash
./pbf_history_reader /path/to/history.osh.pbf /path/to/tag_list.txt
```
Writes to schema `huwise_osm`, table `fr_history`.

### With country code (multi-country support)
```bash
./pbf_history_reader /path/to/history.osh.pbf /path/to/tag_list.txt nz
```
Writes to schema `huwise_osm`, table `nz_history`.
```bash
./pbf_history_reader /path/to/history.osh.pbf /path/to/tag_list.txt au
```
Writes to schema `huwise_osm`, table `au_history`.

### With explicit schema (optional)
```bash
./pbf_history_reader /path/to/history.osh.pbf /path/to/tag_list.txt fr huwise_osm
```

### To know the version
```bash
./pbf_history_reader --version
```

---

## Configure your rust environment

### Tools

Install `rust` and `cargo` with:
```bash
curl https://sh.rustup.rs -sSf | sh
```

### Useful commands

[Documentation](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

#### Building the executable file
from the `osm/pbf_history_reader` subfolder
```
cargo build
```

It creates an executable file in `./target/debug/` directory (by default, `cargo` builds in debug mode)

#### Building & running the executable file
```
cargo run
```

This command builds then run the executable file.

#### Building for release
```
cargo build --release
```

It creates an executable file in `./target/release/` directory. With this option, the program is optimized to run
faster.

---

## Cross-compilation (recommended method)
 
The modern and most robust approach on macOS is to use [Zig](https://ziglang.org/) as a linker via `cargo-zigbuild`. Zig natively supports all cross-compilation targets (glibc and musl) without requiring large GCC toolchains or dealing with dependency issues.
 
### Install tools
 
```bash
brew install zig
cargo install cargo-zigbuild@0.21.8
```
 
### Add the target
 
For **arm64** (Mac M1/M2, arm64 Linux containers):
```bash
rustup target add aarch64-unknown-linux-musl
```
 
For **x86_64** (Intel Linux containers):
```bash
rustup target add x86_64-unknown-linux-gnu
```
 
### Compile (in release)
 
For **arm64**:
```bash
cargo zigbuild --target aarch64-unknown-linux-musl --release
```
The executable is created in `target/aarch64-unknown-linux-musl/release/` directory.
 
For **x86_64**:
```bash
cargo zigbuild --target x86_64-unknown-linux-gnu --release
```
The executable is created in `target/x86_64-unknown-linux-gnu/release/` directory.

---

## Environment variables

pbf_history_reader needs several environment variables to work:
- `DB_HOST` (required)
- `DB_PASSWORD` (required)
- `DB_NAME`
- `DB_PORT` (default: `5432`)
- `DB_USER`
- `OSM_ACCOUNT_USER` (for history files)
- `OSM_ACCOUNT_PASSWORD` (for history files)

They can be provided with `env.local` or `.envrc` files (don't forget to pass them to the container
in `docker-compose.yml` file)

---

## Release

### How to release a new version

1. **Update the version tag in `.github/workflows/rust.yml` and in `Cargo.toml`**
   
   Edit the `tag_name` and `release_name` fields:
```yaml
   - name: Create Release
     uses: softprops/action-gh-release@v1
     with:
       files: target/x86_64-unknown-linux-musl/release/*
       tag_name: v1.2.0        # ← update this
       release_name: Release v1.2.0  # ← and this
```

   Edit the `version` fields:
```
   version = "1.2.0". # <- update this
```

2. **Commit and push to a branch for PR**
```bash
   git add .github/workflows/rust.yml
   git commit -m "chore: bump version to v1.2.0"
   git push origin my_branch
```

3. **GitHub Actions will automatically:**
   - Build the binary for `x86_64-unknown-linux-musl`
   - Create a GitHub Release `v1.3.0` with the binary attached
   - The binary will be available at:
```
     https://github.com/opendatasoft/osm-pbf-history-reader/releases/download/v1.2.0/pbf_history_reader
```