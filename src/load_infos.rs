use crate::infos::GatheredInfos;
use itertools::Itertools;
use postgres::binary_copy::BinaryCopyInWriter;
use postgres::types::{ToSql, Type};
use postgres::Client;
use std::collections::HashMap;

/// Write the content of nodes_info into a PostgreSQL table
pub fn load(
    client: &mut Client,
    schema: &str,
    table: &str,
    nodes_info: HashMap<i64, GatheredInfos>,
) {
    let mut transaction = client
        .transaction()
        .expect("Failed to begin a new transaction");
    let copy_in_writer = transaction
        .copy_in(&format!("COPY {}.{} FROM STDIN BINARY", schema, table))
        .expect("Failed to create COPY IN writer");

    let types = vec![
        Type::INT8,
        Type::TIMESTAMPTZ_ARRAY,
        Type::INT8_ARRAY,
        Type::TIMESTAMPTZ,
        Type::TIMESTAMPTZ,
        Type::INT8,
        Type::INT8,
    ];
    let mut writer = BinaryCopyInWriter::new(copy_in_writer, &types);

    for (key, value) in nodes_info.iter() {
        let mut row: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let first_update = value.timestamps.iter().min().unwrap();
        let last_update = value.timestamps.iter().max().unwrap();
        let users_number = value.uids.iter().unique().count() as i64;
        let versions_number = value.timestamps.len() as i64;
        row.push(key);
        row.push(&value.timestamps);
        row.push(&value.changesets);
        row.push(first_update);
        row.push(last_update);
        row.push(&users_number);
        row.push(&versions_number);
        writer.write(&row).unwrap();
    }
    writer.finish().unwrap();

    transaction.commit().unwrap();

    println!("\n{} rows copied in", nodes_info.len());
}
