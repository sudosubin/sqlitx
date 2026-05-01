use colored::Color;

pub fn icon_for(db_type: &str) -> &'static str {
    match db_type {
        "athena" => "",
        "bigquery" | "spanner" => "󰊭",
        "mariadb" | "mysql" => "",
        "mssql" | "oracle" | "oracle_legacy" => "󰡦",
        "postgresql" | "cockroachdb" | "redshift" | "supabase" => "",
        "sqlite" | "duckdb" | "turso" | "d1" | "motherduck" => "󰆼",
        "snowflake" => "󰜗",
        "clickhouse" | "trino" | "presto" | "db2" | "firebird" | "hana" | "teradata"
        | "flight" | "impala" | "osquery" | "surrealdb" => "󰆼",
        _ => "󰆼",
    }
}

pub fn color_for(db_type: &str) -> Color {
    match db_type {
        "mysql" => Color::Yellow,
        "mariadb" => Color::Yellow,
        "postgresql" => Color::Blue,
        "cockroachdb" => Color::Blue,
        "redshift" => Color::Red,
        "supabase" => Color::Green,
        "sqlite" => Color::Blue,
        "duckdb" => Color::Yellow,
        "turso" => Color::Green,
        "d1" => Color::Yellow,
        "motherduck" => Color::Yellow,
        "mssql" => Color::Red,
        "oracle" | "oracle_legacy" => Color::Red,
        "snowflake" => Color::Blue,
        "bigquery" | "spanner" => Color::Blue,
        "clickhouse" => Color::Yellow,
        "trino" | "presto" => Color::Magenta,
        "athena" => Color::Yellow,
        "db2" => Color::Blue,
        "firebird" => Color::Red,
        "hana" => Color::Blue,
        "teradata" => Color::Blue,
        "flight" => Color::Magenta,
        "impala" => Color::Yellow,
        "osquery" => Color::White,
        "surrealdb" => Color::Red,
        _ => Color::White,
    }
}
