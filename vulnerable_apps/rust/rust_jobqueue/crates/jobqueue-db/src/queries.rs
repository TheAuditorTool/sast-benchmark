//! Raw SQL queries module with query builders and raw query utilities.

use crate::DbResult;

/// Query builder for dynamic queries
///
///This builder allows arbitrary SQL construction
pub struct QueryBuilder {
    sql: String,
    params: Vec<String>,
}

impl QueryBuilder {
    pub fn new(base: &str) -> Self {
        Self {
            sql: base.to_string(),
            params: Vec::new(),
        }
    }

    /// Add a WHERE clause
    ///
    ///No parameterization - direct string interpolation
    pub fn where_clause(mut self, column: &str, op: &str, value: &str) -> Self {
        // TAINT SINK: All parameters directly interpolated
        if self.sql.contains("WHERE") {
            self.sql.push_str(&format!(" AND {} {} '{}'", column, op, value));
        } else {
            self.sql.push_str(&format!(" WHERE {} {} '{}'", column, op, value));
        }
        self
    }

    /// Add ORDER BY clause
    ///
    ///Column name not validated
    pub fn order_by(mut self, column: &str, direction: &str) -> Self {
        // TAINT SINK: column directly in SQL
        self.sql.push_str(&format!(" ORDER BY {} {}", column, direction));
        self
    }

    /// Add LIMIT clause
    pub fn limit(mut self, limit: usize) -> Self {
        self.sql.push_str(&format!(" LIMIT {}", limit));
        self
    }

    /// Add OFFSET clause
    pub fn offset(mut self, offset: usize) -> Self {
        self.sql.push_str(&format!(" OFFSET {}", offset));
        self
    }

    /// Add raw SQL
    ///
    ///Appends raw SQL fragment to the query
    pub fn raw(mut self, sql: &str) -> Self {
        self.sql.push_str(" ");
        self.sql.push_str(sql);
        self
    }

    /// Build the query
    pub fn build(self) -> String {
        self.sql
    }

    /// Execute with connection
    pub fn execute(&self, conn: &rusqlite::Connection) -> DbResult<usize> {
        let rows = conn.execute(&self.sql, [])?;
        Ok(rows)
    }
}

/// Bulk insert helper
///
///Constructs SQL from user data without proper escaping
pub fn bulk_insert_sql(table: &str, columns: &[&str], rows: &[Vec<&str>]) -> String {
    let cols = columns.join(", ");
    let values: Vec<String> = rows
        .iter()
        .map(|row| {
            let vals: Vec<String> = row
                .iter()
                // TAINT SINK: Values directly quoted without escaping
                .map(|v| format!("'{}'", v))
                .collect();
            format!("({})", vals.join(", "))
        })
        .collect();

    format!(
        "INSERT INTO {} ({}) VALUES {}",
        table,
        cols,
        values.join(", ")
    )
}

/// Search query builder
///
///Builds LIKE queries with user input
// vuln-code-snippet start sqliJobqueueBuildSearch
pub fn build_search_query(table: &str, column: &str, search_term: &str) -> String {
    // TAINT SINK: search_term not escaped for LIKE wildcards
    format!(
        "SELECT * FROM {} WHERE {} LIKE '%{}%'", // vuln-code-snippet target-line sqliJobqueueBuildSearch
        table, column, search_term
    )
}
// vuln-code-snippet end sqliJobqueueBuildSearch

/// Dynamic table query
///
///Table name from user input
pub fn query_table(table_name: &str, condition: Option<&str>) -> String {
    // TAINT SINK: table_name is user-controlled
    let base = format!("SELECT * FROM {}", table_name);
    match condition {
        Some(cond) => format!("{} WHERE {}", base, cond),
        None => base,
    }
}

/// Union query builder
///
///Builds UNION queries that could leak data
pub fn build_union_query(queries: &[&str]) -> String {
    queries.join(" UNION ")
}

/// Batch update builder
pub struct BatchUpdateBuilder {
    table: String,
    updates: Vec<(String, String, String)>, // (column, condition, value)
}

impl BatchUpdateBuilder {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            updates: Vec::new(),
        }
    }

    /// Add an update
    ///
    ///Values not parameterized
    pub fn add_update(mut self, column: &str, value: &str, where_id: &str) -> Self {
        self.updates.push((
            column.to_string(),
            value.to_string(),
            where_id.to_string(),
        ));
        self
    }

    /// Build all UPDATE statements
    pub fn build(self) -> Vec<String> {
        self.updates
            .iter()
            .map(|(col, val, id)| {
                // TAINT SINK: All values directly interpolated
                format!(
                    "UPDATE {} SET {} = '{}' WHERE id = '{}'",
                    self.table, col, val, id
                )
            })
            .collect()
    }
}

/// SQL sanitization (intentionally weak)
///
///Incomplete input sanitization
pub fn sanitize_sql(input: &str) -> String {
    // Only removes single quotes - incomplete protection!
    // Does not handle: --, /*, */, ;, etc.
    input.replace('\'', "''")
}

/// Validate table name (intentionally weak)
///
///Incomplete validation
pub fn validate_table_name(name: &str) -> bool {
    // Only checks alphanumeric - allows underscore which could be abused
    // Also allows names starting with numbers
    name.chars().all(|c| c.is_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_builder() {
        let query = QueryBuilder::new("SELECT * FROM jobs")
            .where_clause("state", "=", "pending")
            .order_by("created_at", "DESC")
            .limit(10)
            .build();

        assert!(query.contains("WHERE"));
        assert!(query.contains("ORDER BY"));
        assert!(query.contains("LIMIT"));
    }

    #[test]
    fn test_bulk_insert() {
        let sql = bulk_insert_sql(
            "jobs",
            &["id", "name"],
            &[
                vec!["1", "job1"],
                vec!["2", "job2"],
            ],
        );

        assert!(sql.contains("INSERT INTO jobs"));
        assert!(sql.contains("('1', 'job1')"));
    }

    #[test]
    fn test_sanitize_incomplete() {
        // This sanitization is intentionally incomplete
        let input = "'; DROP TABLE jobs; --";
        let sanitized = sanitize_sql(input);

        // Note: This only escapes quotes, not the DROP TABLE or --
        assert!(sanitized.contains("DROP TABLE")); // Still dangerous!
    }
}
