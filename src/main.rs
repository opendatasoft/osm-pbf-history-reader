use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

mod history_processing;
mod infos;
mod load_infos;
mod postgres_client;

// Used to build a table name in the format: <country_code>_<suffix>.
const TABLE_SUFFIX: &str = "history";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--version" {
        println!("pbf_history_reader version: {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    let host: String = env::var("DB_HOST").expect("$DB_HOST is not set");
    let password: String = env::var("DB_PASSWORD").expect("$DB_PASSWORD is not set");
    let user: String = env::var("DB_USER").unwrap_or(String::from("dataseed"));
    let dbname: String = env::var("DB_NAME").unwrap_or(String::from("dataseed"));
    let port: String = env::var("DB_PORT").unwrap_or(String::from("5432"));

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Usage: {} <history_pbf> <tag_list> [country_code] [schema] (or --version)", args[0]);
    }

    let history_file_path_str = &args[1];
    let history_file_path = Path::new(history_file_path_str);

    let tag_list_file_path_str = &args[2];
    let tag_list_file_path = Path::new(tag_list_file_path_str);

    // If no country_code is provided, defaults to "fr"
    let country_code = args.get(3).cloned().unwrap_or_else(|| String::from("fr"));

    if country_code.len() != 2 {
        panic!(
            "Invalid country_code '{}': must be exactly 2 letters (e.g., 'fr', 'au', 'nz')",
            country_code
        );
    }

    let schema = args.get(4)
        .cloned()
        .unwrap_or_else(|| String::from("huwise_osm"));

    for path in [history_file_path, tag_list_file_path] {
        if !path.exists() {
            panic!("File {} does not exist", path.display())
        }
    }

    // The table name, e.g. "fr_history" for country code "fr"
    let tablename = format!("{}_{}", country_code.to_ascii_lowercase(), TABLE_SUFFIX);

    // Read tag_list file and store tags into an Hashset (ignore values after the equal signs)
    let file = File::open(tag_list_file_path).expect("Failed to open tag list file.");
    let reader = io::BufReader::new(file);
    let mut tag_list_string: Vec<String> = Vec::new();
    for line in reader.lines() {
        let final_line = match line {
            Ok(line) => {
                if let Some(index) = line.find('=') {
                    line[..index].to_owned()
                } else {
                    line.to_owned()
                }
            }
            Err(_) => String::new(),
        };
        tag_list_string.push(final_line);
    }
    let tag_list: HashSet<&str> = tag_list_string.iter().map(|s| s.as_str()).collect();

    println!("Tags to filter: {:?}", tag_list);
    println!("Processing for country: {} (SQL schema: {})", country_code, schema);

    let now = Instant::now();
    let mut db_client = postgres_client::connect(&host, &user, &password, &dbname, &port);
    let db_connection_time = Instant::now();

    postgres_client::create_schema(&mut db_client, &schema);
    let db_schema_creation_time = Instant::now();

    postgres_client::create_table_history(&mut db_client, &schema, &tablename);
    let db_history_creation_time = Instant::now();

    let elements_infos =
        history_processing::process_history(history_file_path.to_str().unwrap(), tag_list);
    let process_history_time = Instant::now();

    postgres_client::add_index(&mut db_client, &schema, &tablename, "id");
    let indexing_time = Instant::now();

    load_infos::load(&mut db_client, &schema, &tablename, elements_infos);
    let insert_history_time = Instant::now();

    println!(
        "\nConnection time:\t{:?}\n\
        Schema creation time:\t{:?}\n\
        History table creation time:\t{:?}\n\
        History processing time:\t{:?}\n\
        Indexing time:\t{:?}\n\
        History insertion time:\t{:?}\n\
        Total:\t{:?}
        ",
        db_connection_time.duration_since(now),
        db_schema_creation_time.duration_since(db_connection_time),
        db_history_creation_time.duration_since(db_schema_creation_time),
        process_history_time.duration_since(db_history_creation_time),
        indexing_time.duration_since(process_history_time),
        insert_history_time.duration_since(indexing_time),
        insert_history_time.duration_since(now),
    )
}