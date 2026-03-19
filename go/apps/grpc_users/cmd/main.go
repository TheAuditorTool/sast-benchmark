package main

import (
	"database/sql"
	"log"
	"net"

	_ "github.com/mattn/go-sqlite3"
	"github.com/theauditor/grpc_users/server"
	"google.golang.org/grpc"
)

func main() {
	// Initialize database
	db, err := sql.Open("sqlite3", "./users.db")
	if err != nil {
		log.Fatalf("Failed to open database: %v", err)
	}
	defer db.Close()

	// Create tables
	initDB(db)

	// Create gRPC server
	listener, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("Failed to listen: %v", err)
	}

	grpcServer := grpc.NewServer()

	// Register user service
	userServer := server.NewUserServer(db)
	// In real code: pb.RegisterUserServiceServer(grpcServer, userServer)
	_ = userServer

	log.Println("gRPC server listening on :50051")
	if err := grpcServer.Serve(listener); err != nil {
		log.Fatalf("Failed to serve: %v", err)
	}
}

func initDB(db *sql.DB) {
	schema := `
	CREATE TABLE IF NOT EXISTS users (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		username TEXT NOT NULL,
		email TEXT NOT NULL,
		password TEXT,
		role TEXT DEFAULT 'user',
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);

	CREATE TABLE IF NOT EXISTS user_metadata (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		user_id TEXT NOT NULL,
		key TEXT NOT NULL,
		value TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);
	`
	db.Exec(schema)
}
