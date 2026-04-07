package main

import (
	"database/sql"
	"log"

	beego "github.com/beego/beego/v2/server/web"
	_ "github.com/mattn/go-sqlite3"
	"github.com/theauditor/beego_admin/routers"
)

func main() {
	// Initialize database
	db, err := sql.Open("sqlite3", "./admin.db")
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	// Create tables
	initDB(db)

	// Store db in app config for controllers to access
	beego.BConfig.AppName = "beego_admin"
	beego.BConfig.RunMode = "dev"

	// Initialize routes
	routers.Init()

	// Run server
	beego.Run(":8080")
}

func initDB(db *sql.DB) {
	schema := `
	CREATE TABLE IF NOT EXISTS users (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		username TEXT NOT NULL,
		email TEXT NOT NULL,
		password TEXT,
		role TEXT DEFAULT 'user',
		api_key TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);

	CREATE TABLE IF NOT EXISTS audit_log (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		user_id INTEGER,
		action TEXT,
		resource TEXT,
		details TEXT,
		ip_address TEXT,
		user_agent TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);

	CREATE TABLE IF NOT EXISTS sessions (
		id TEXT PRIMARY KEY,
		user_id INTEGER,
		data TEXT,
		expires_at DATETIME
	);

	CREATE TABLE IF NOT EXISTS reports (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		name TEXT,
		query TEXT,
		output TEXT,
		created_by INTEGER,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);

	CREATE TABLE IF NOT EXISTS jobs (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		name TEXT,
		status TEXT DEFAULT 'pending',
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);

	CREATE TABLE IF NOT EXISTS user_inputs (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		user_id INTEGER,
		input TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);
	`
	db.Exec(schema)
}
