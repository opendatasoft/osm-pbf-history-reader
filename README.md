# OpenStreetMap PBF history parser
This Rust program:
- parses OSM [history files from Geofabrik](https://osm-internal.download.geofabrik.de)
- then creates a postgres `osm.history` table with the following fields:
  - id (negative IDs are for relations)
  - timestamps
  - changesets
  - first_timestamps
  - last_timestamps
  - users_number
  - versions_number


## Configure your rust environment
### Tools

Install `rust` and `cargo` with:

```
curl https://sh.rustup.rs -sSf | sh
```

### Useful commands

[Documentation](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

#### Building the executable file
from the `osm/pbf_history_reader` subfolder
````
cargo build
````

It creates an executable file in `./target/debug/` directory (by default, `cargo` builds in debug mode)

#### Building & running the executable file

````
cargo run
````

This command builds then run the executable file.

#### Building for release

```
cargo build --release
```

It creates an executable file in `./target/release/` directory. With this option, the program is optimized to run
faster.

### Cross-compilation

- Install a cross toolchain:

On mac:

```
brew tap SergioBenitez/osxct
brew install x86_64-unknown-linux-gnu
```

then specify it in cargo config file `.cargo/config.toml`:

```bash
# .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "/opt/homebrew/bin/x86_64-unknown-linux-gnu-gcc"
```

- Specify the compilation target:

```
rustup target add x86_64-unknown-linux-gnu
```

- Compile (in release):

```
cargo build --target x86_64-unknown-linux-gnu -v --release
```

The executable file is created in `osm/pbf_history_reader/target/x86_64-unknown-linux-gnu/release` directory

> pbf_history_reader needs several environment variables to work:
> - DB_NAME
> - DB_PORT
> - DB_USER
> 
> (DB_NAME, DB_USER and DB_PORT have default values defined in `osm/pbf_history_reader/src/main.rs` file, but you can
> override them (especially if you are working locally))
> - DB_HOST
> - DB_PASSWORD
> - OSM_ACCOUNT_PASSWORD (for history files)
> - OSM_ACCOUNT_USER
>

> They can be provided with `env.local` or `.envrc` files (don't forget to pass them to the container
> in `docker-compose.yml` file)
