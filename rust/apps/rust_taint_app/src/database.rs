//! Database operations with SQL query construction patterns.

use crate::models::{User, UserInput, UserSearchQuery};
use sqlx::{Row, SqlitePool};

///Parameterized query using sqlx
pub async fn get_all_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    //No user input in query
    let rows = sqlx::query("SELECT id, username, email, password_hash, role, created_at FROM users")
        .fetch_all(pool)
        .await?;

    let users: Vec<User> = rows
        .iter()
        .map(|row| User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            role: row.get("role"),
            created_at: row.get("created_at"),
        })
        .collect();

    Ok(users)
}

// vuln-code-snippet start sqliGetUserById
///Parameterized query with bind parameter
pub async fn get_user_by_id(pool: &SqlitePool, user_id: i64) -> Result<Option<User>, sqlx::Error> {
    //Using bind parameter
    let row = sqlx::query("SELECT id, username, email, password_hash, role, created_at FROM users WHERE id = ?")
        .bind(user_id) // vuln-code-snippet target-line sqliGetUserById
        .fetch_optional(pool)
        .await?;

    Ok(row.map(|r| User {
        id: r.get("id"),
        username: r.get("username"),
        email: r.get("email"),
        password_hash: r.get("password_hash"),
        role: r.get("role"),
        created_at: r.get("created_at"),
    }))
}
// vuln-code-snippet end sqliGetUserById

// vuln-code-snippet start sqliCreateUser
///Parameterized insert
pub async fn create_user(pool: &SqlitePool, input: &UserInput) -> Result<User, sqlx::Error> {
    //Using bind parameters
    let result = sqlx::query(
        "INSERT INTO users (username, email, password_hash, role, created_at) VALUES (?, ?, ?, ?, datetime('now'))"
    )
    .bind(&input.username) // vuln-code-snippet target-line sqliCreateUser
    .bind(&input.email)
    .bind(&input.password)  // In real app, this would be hashed
    .bind(input.role.as_deref().unwrap_or("user"))
    .execute(pool)
    .await?;

    let user_id = result.last_insert_rowid();

    get_user_by_id(pool, user_id).await.map(|opt| opt.unwrap())
}
// vuln-code-snippet end sqliCreateUser

// vuln-code-snippet start sqliSearchUsers
///SQL injection via string concatenation!
/// TAINT SINK: User input directly in SQL query
pub async fn search_users_vulnerable(
    pool: &SqlitePool,
    params: &UserSearchQuery,
) -> Result<Vec<User>, sqlx::Error> {
    //String concatenation with user input!
    let mut query = String::from("SELECT id, username, email, password_hash, role, created_at FROM users WHERE 1=1");

    // TAINT FLOW: query param -> SQL query (SQL INJECTION!)
    if let Some(ref name) = params.name {
        query.push_str(&format!(" AND username LIKE '%{}%'", name)); // vuln-code-snippet target-line sqliSearchUsers
    }

    // TAINT FLOW: query param -> SQL query (SQL INJECTION!)
    if let Some(ref email) = params.email {
        query.push_str(&format!(" AND email LIKE '%{}%'", email));
    }

    // TAINT FLOW: query param -> SQL ORDER BY (SQL INJECTION!)
    if let Some(ref sort_by) = params.sort_by {
        query.push_str(&format!(" ORDER BY {}", sort_by));

        if let Some(ref order) = params.order {
            query.push_str(&format!(" {}", order));
        }
    }

    // TAINT FLOW: query param -> SQL LIMIT (SQL INJECTION!)
    if let Some(limit) = params.limit {
        query.push_str(&format!(" LIMIT {}", limit));
    }

    // TAINT SINK: Execute vulnerable query
    let rows = sqlx::query(&query).fetch_all(pool).await?;

    let users: Vec<User> = rows
        .iter()
        .map(|row| User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            role: row.get("role"),
            created_at: row.get("created_at"),
        })
        .collect();

    Ok(users)
}
// vuln-code-snippet end sqliSearchUsers

// vuln-code-snippet start sqliExecuteRawSql
///Execute arbitrary SQL
/// TAINT SINK: sqlx::query with user-controlled query string
pub async fn execute_raw_sql(pool: &SqlitePool, sql: &str) -> Result<u64, sqlx::Error> {
    // TAINT SINK: Direct SQL execution (CRITICAL!)
    let result = sqlx::query(sql).execute(pool).await?; // vuln-code-snippet target-line sqliExecuteRawSql
    Ok(result.rows_affected())
}
// vuln-code-snippet end sqliExecuteRawSql

// vuln-code-snippet start sqliFindUserByEmail
///Using query_as with string interpolation
pub async fn find_user_by_email_vulnerable(
    pool: &SqlitePool,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    //String interpolation in query
    let query = format!(
        "SELECT id, username, email, password_hash, role, created_at FROM users WHERE email = '{}'", // vuln-code-snippet target-line sqliFindUserByEmail
        email
    );

    // TAINT SINK: sqlx::query with interpolated string
    let row = sqlx::query(&query).fetch_optional(pool).await?;

    Ok(row.map(|r| User {
        id: r.get("id"),
        username: r.get("username"),
        email: r.get("email"),
        password_hash: r.get("password_hash"),
        role: r.get("role"),
        created_at: r.get("created_at"),
    }))
}
// vuln-code-snippet end sqliFindUserByEmail

/// Example using rusqlite (sync database)
pub mod rusqlite_ops {
    use rusqlite::{Connection, Result};
    use crate::models::UserRow;

    // vuln-code-snippet start sqliRusqliteGetUser
    ///Parameterized query
    pub fn get_user_safe(conn: &Connection, user_id: i64) -> Result<UserRow> {
        conn.query_row(
            "SELECT id, username, email FROM users WHERE id = ?",
            [user_id], // vuln-code-snippet target-line sqliRusqliteGetUser
            |row| {
                Ok(UserRow {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    email: row.get(2)?,
                })
            },
        )
    }
    // vuln-code-snippet end sqliRusqliteGetUser

    // vuln-code-snippet start sqliRusqliteDelete
    ///SQL injection via string concatenation
    /// TAINT SINK: rusqlite::Connection::execute with user input
    pub fn delete_user_vulnerable(conn: &Connection, username: &str) -> Result<usize> {
        //String interpolation
        let sql = format!("DELETE FROM users WHERE username = '{}'", username); // vuln-code-snippet target-line sqliRusqliteDelete

        // TAINT SINK: Direct SQL execution
        conn.execute(&sql, [])
    }
    // vuln-code-snippet end sqliRusqliteDelete

    // vuln-code-snippet start sqliRusqliteSearch
    ///Search with SQL injection
    pub fn search_users_vulnerable(conn: &Connection, search_term: &str) -> Result<Vec<UserRow>> {
        //String concatenation
        let sql = format!(
            "SELECT id, username, email FROM users WHERE username LIKE '%{}%'", // vuln-code-snippet target-line sqliRusqliteSearch
            search_term
        );

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map([], |row| {
            Ok(UserRow {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
            })
        })?;

        rows.collect()
    }
    // vuln-code-snippet end sqliRusqliteSearch
}

/// Example using diesel ORM
pub mod diesel_ops {
    // Note: Diesel queries are typically safe due to query builder
    // But raw SQL is still possible

    ///Using diesel::sql_query with raw SQL
    pub fn execute_raw_diesel(sql: &str) {
        // This would be: diesel::sql_query(sql).execute(conn)
        // TAINT SINK: Raw SQL execution
        println!("Would execute: {}", sql);
    }
}
