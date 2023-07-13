use postgres::{Client, NoTls};

pub fn connect(host: &str, user: &str, password: &str, dbname: &str, port: &str) -> Client {
    match Client::connect(
        &format!(
            "host={} user={} password={} dbname={} port={}",
            host, user, password, dbname, port
        ),
        NoTls,
    ) {
        Ok(client) => {
            println!("Client connected to database");
            client
        }
        Err(e) => panic!("{}",format!("Client failed to connect to the database with config: host={} user={} dbname={} port={} : {}",
            host, user, dbname, port, e))
    }
}

pub fn create_schema(client: &mut Client, schema: &str) {
    match client.batch_execute(&format!("CREATE SCHEMA IF NOT EXISTS {}", schema)) {
        Ok(_) => {
            println!("Schema {} created", schema);
        }
        Err(e) => panic!("Client failed to create {} schema: {}", schema, e),
    };
}

pub fn add_index(client: &mut Client, schema: &str, table: &str, column: &str) {
    match client.batch_execute(&format!(
        "CREATE INDEX IF NOT EXISTS IX_{}_{} ON {}.{} USING btree ({});",
        table, column, schema, table, column
    )) {
        Ok(_) => {
            println!(
                "Index IX_{}_{} on {}.{} created",
                table, column, schema, table
            );
        }
        Err(e) => panic!(
            "Client failed to create index on {}.{}: {}",
            table, column, e
        ),
    };
}

pub fn create_table_history(client: &mut Client, schema: &str, table: &str) {
    drop_table(client, schema, table);
    match client.batch_execute(&format!(
        "CREATE TABLE {}.{} (
            id int8,
            timestamps TIMESTAMP WITH TIME ZONE[],
            changesets int8[],
            first_timestamps TIMESTAMP WITH TIME ZONE,
            last_timestamps TIMESTAMP WITH TIME ZONE,
            users_number int8,
            versions_number int8)",
        schema, table
    )) {
        Ok(_) => {
            println!("Table {}.{} created", schema, table);
        }
        Err(e) => panic!("Client failed to create {}.{} table: {}", schema, table, e),
    };
}

pub fn drop_table(client: &mut Client, schema: &str, table: &str) {
    match client.batch_execute(&format!("DROP TABLE IF EXISTS {}.{}", schema, table)) {
        Ok(_) => {
            println!("Table {}.{} dropped", schema, table);
        }
        Err(e) => panic!("Client failed to drop {}.{} table: {}", schema, table, e),
    };
}
