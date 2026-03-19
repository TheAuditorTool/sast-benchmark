// Package storage provides database operations for notifications
package storage

import (
	"database/sql"
	"fmt"
	"log"
	"os/exec"
	"strings"
	"time"

	_ "github.com/mattn/go-sqlite3"
	"github.com/project-anarchy/go_notifications/internal/channels"
)

// SQLiteStore handles SQLite database operations
type SQLiteStore struct {
	db *sql.DB
}

// NewSQLiteStore creates a new SQLite store
func NewSQLiteStore(dbPath string) (*SQLiteStore, error) {
	db, err := sql.Open("sqlite3", dbPath)
	if err != nil {
		return nil, fmt.Errorf("failed to open database: %w", err)
	}

	store := &SQLiteStore{db: db}

	if err := store.init(); err != nil {
		return nil, fmt.Errorf("failed to initialize database: %w", err)
	}

	return store, nil
}

// init creates required tables
func (s *SQLiteStore) init() error {
	schema := `
	CREATE TABLE IF NOT EXISTS notifications (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		channel TEXT NOT NULL,
		recipient TEXT NOT NULL,
		subject TEXT,
		message TEXT,
		status TEXT DEFAULT 'pending',
		error TEXT,
		metadata TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		sent_at DATETIME
	);

	CREATE TABLE IF NOT EXISTS jobs (
		id TEXT PRIMARY KEY,
		data TEXT NOT NULL,
		status TEXT DEFAULT 'pending',
		error TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);

	CREATE TABLE IF NOT EXISTS users (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		username TEXT UNIQUE NOT NULL,
		email TEXT NOT NULL,
		password TEXT NOT NULL,
		role TEXT DEFAULT 'user',
		api_key TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);

	CREATE TABLE IF NOT EXISTS templates (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		name TEXT UNIQUE NOT NULL,
		content TEXT NOT NULL,
		created_by TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);

	CREATE INDEX IF NOT EXISTS idx_notifications_status ON notifications(status);
	CREATE INDEX IF NOT EXISTS idx_notifications_channel ON notifications(channel);
	CREATE INDEX IF NOT EXISTS idx_jobs_status ON jobs(status);
	`

	_, err := s.db.Exec(schema)
	return err
}

// Close closes the database connection
func (s *SQLiteStore) Close() error {
	return s.db.Close()
}

// Ping checks database connectivity
func (s *SQLiteStore) Ping() string {
	if err := s.db.Ping(); err != nil {
		return "unhealthy: " + err.Error()
	}
	return "healthy"
}

// SaveNotification stores a notification
func (s *SQLiteStore) SaveNotification(n *channels.Notification) (int64, error) {
	result, err := s.db.Exec(`
		INSERT INTO notifications (channel, recipient, subject, message, status, metadata)
		VALUES (?, ?, ?, ?, 'pending', ?)
	`, n.Channel, n.Recipient, n.Subject, n.Message, metadataToJSON(n.Metadata))
	if err != nil {
		return 0, err
	}

	return result.LastInsertId()
}

// UpdateStatus updates notification status
func (s *SQLiteStore) UpdateStatus(id int64, status, errorMsg string) error {
	var query string
	if status == "sent" {
		query = `UPDATE notifications SET status = ?, sent_at = CURRENT_TIMESTAMP WHERE id = ?`
		_, err := s.db.Exec(query, status, id)
		return err
	}

	query = `UPDATE notifications SET status = ?, error = ? WHERE id = ?`
	_, err := s.db.Exec(query, status, errorMsg, id)
	return err
}

// ListNotifications retrieves notifications with optional filters
// VULN: SQL Injection via orderBy parameter
func (s *SQLiteStore) ListNotifications(channel, status, recipient, limit, orderBy string) ([]map[string]interface{}, error) {
	query := "SELECT id, channel, recipient, subject, message, status, error, created_at, sent_at FROM notifications WHERE 1=1"
	args := []interface{}{}

	// These are parameterized (safe)
	if channel != "" {
		query += " AND channel = ?"
		args = append(args, channel)
	}
	if status != "" {
		query += " AND status = ?"
		args = append(args, status)
	}
	if recipient != "" {
		query += " AND recipient = ?"
		args = append(args, recipient)
	}

	// VULN: SQL Injection - orderBy is concatenated directly
	if orderBy != "" {
		// orderBy could be: "id; DROP TABLE notifications;--"
		query += " ORDER BY " + orderBy // TAINT SINK: SQL Injection
	} else {
		query += " ORDER BY created_at DESC"
	}

	// VULN: SQL Injection - limit is concatenated directly
	if limit != "" {
		// limit could be: "1; DELETE FROM notifications;--"
		query += " LIMIT " + limit // TAINT SINK: SQL Injection
	} else {
		query += " LIMIT 100"
	}

	log.Printf("Executing query: %s", query) // VULN: Logs potentially malicious query

	rows, err := s.db.Query(query, args...)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	return scanNotifications(rows)
}

// Search performs full-text search
// VULN: SQL Injection via search query
func (s *SQLiteStore) Search(searchQuery string) ([]map[string]interface{}, error) {
	// VULN: Search query used directly in LIKE clause
	// searchQuery could be: "'; DROP TABLE notifications;--"
	query := fmt.Sprintf(`
		SELECT id, channel, recipient, subject, message, status, created_at
		FROM notifications
		WHERE subject LIKE '%%%s%%' OR message LIKE '%%%s%%'
		ORDER BY created_at DESC
		LIMIT 100
	`, searchQuery, searchQuery) // TAINT SINK: SQL Injection

	rows, err := s.db.Query(query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	return scanNotifications(rows)
}

// GetNotificationByID retrieves a single notification
func (s *SQLiteStore) GetNotificationByID(id string) (map[string]interface{}, error) {
	// VULN: SQL Injection via ID
	query := fmt.Sprintf("SELECT * FROM notifications WHERE id = %s", id) // TAINT SINK
	row := s.db.QueryRow(query)

	var n channels.Notification
	err := row.Scan(&n.ID, &n.Channel, &n.Recipient, &n.Subject, &n.Message, &n.Status)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{"notification": n}, nil
}

// SaveJob stores a job
func (s *SQLiteStore) SaveJob(id, data string) error {
	_, err := s.db.Exec(`
		INSERT OR REPLACE INTO jobs (id, data, status, updated_at)
		VALUES (?, ?, 'pending', CURRENT_TIMESTAMP)
	`, id, data)
	return err
}

// LoadPendingJobs retrieves pending jobs
func (s *SQLiteStore) LoadPendingJobs() ([]string, error) {
	rows, err := s.db.Query(`
		SELECT data FROM jobs
		WHERE status = 'pending'
		ORDER BY created_at ASC
		LIMIT 100
	`)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var jobs []string
	for rows.Next() {
		var data string
		if err := rows.Scan(&data); err != nil {
			continue
		}
		jobs = append(jobs, data)
	}

	return jobs, nil
}

// UpdateJobStatus updates job status
func (s *SQLiteStore) UpdateJobStatus(jobID, status, errorMsg string) error {
	_, err := s.db.Exec(`
		UPDATE jobs SET status = ?, error = ?, updated_at = CURRENT_TIMESTAMP
		WHERE id = ?
	`, status, errorMsg, jobID)
	return err
}

// GetJobStatus returns job status
func (s *SQLiteStore) GetJobStatus(jobID string) (string, error) {
	var status string
	err := s.db.QueryRow("SELECT status FROM jobs WHERE id = ?", jobID).Scan(&status)
	return status, err
}

// CreateUser creates a new user
// VULN: Weak password handling
func (s *SQLiteStore) CreateUser(username, email, password string) error {
	// VULN: Password stored in plaintext
	_, err := s.db.Exec(`
		INSERT INTO users (username, email, password, api_key)
		VALUES (?, ?, ?, ?)
	`, username, email, password, generateAPIKey()) // VULN: Plaintext password

	return err
}

// AuthenticateUser checks user credentials
// VULN: SQL Injection and timing attack
func (s *SQLiteStore) AuthenticateUser(username, password string) (bool, error) {
	// VULN: SQL Injection via username
	query := fmt.Sprintf("SELECT password FROM users WHERE username = '%s'", username) // TAINT SINK
	var storedPassword string
	err := s.db.QueryRow(query).Scan(&storedPassword)
	if err != nil {
		return false, err
	}

	// VULN: String comparison (timing attack)
	// VULN: Comparing plaintext passwords
	return storedPassword == password, nil
}

// GetUserByAPIKey retrieves user by API key
// VULN: SQL Injection
func (s *SQLiteStore) GetUserByAPIKey(apiKey string) (map[string]interface{}, error) {
	// VULN: SQL Injection via API key
	query := fmt.Sprintf("SELECT id, username, email, role FROM users WHERE api_key = '%s'", apiKey)
	row := s.db.QueryRow(query) // TAINT SINK

	var id int
	var username, email, role string
	if err := row.Scan(&id, &username, &email, &role); err != nil {
		return nil, err
	}

	return map[string]interface{}{
		"id":       id,
		"username": username,
		"email":    email,
		"role":     role,
	}, nil
}

// SaveTemplate stores a template
// VULN: Template content stored without sanitization
func (s *SQLiteStore) SaveTemplate(name, content, createdBy string) error {
	_, err := s.db.Exec(`
		INSERT OR REPLACE INTO templates (name, content, created_by)
		VALUES (?, ?, ?)
	`, name, content, createdBy) // VULN: Malicious template content stored
	return err
}

// GetTemplate retrieves a template
func (s *SQLiteStore) GetTemplate(name string) (string, error) {
	// VULN: SQL Injection via template name
	query := fmt.Sprintf("SELECT content FROM templates WHERE name = '%s'", name) // TAINT SINK
	var content string
	err := s.db.QueryRow(query).Scan(&content)
	return content, err
}

// DeleteNotification deletes a notification by ID
// VULN: SQL Injection
func (s *SQLiteStore) DeleteNotification(id string) error {
	query := fmt.Sprintf("DELETE FROM notifications WHERE id = %s", id) // TAINT SINK
	_, err := s.db.Exec(query)
	return err
}

// BulkDelete deletes multiple notifications
// VULN: SQL Injection via IDs
func (s *SQLiteStore) BulkDelete(ids []string) error {
	// VULN: IDs joined directly into query
	idList := strings.Join(ids, ",")
	query := fmt.Sprintf("DELETE FROM notifications WHERE id IN (%s)", idList) // TAINT SINK
	_, err := s.db.Exec(query)
	return err
}

// ExecRawQuery executes a raw SQL query
// VULN: Direct SQL execution
func (s *SQLiteStore) ExecRawQuery(query string) ([]map[string]interface{}, error) {
	// VULN: Arbitrary SQL execution
	rows, err := s.db.Query(query) // TAINT SINK: Direct SQL injection
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	return scanToMaps(rows)
}

// Backup creates a database backup
// VULN: Command injection via backup path
func (s *SQLiteStore) Backup(backupPath string) error {
	// VULN: Path in shell command
	cmdStr := fmt.Sprintf("sqlite3 notifications.db '.backup %s'", backupPath)
	cmd := exec.Command("sh", "-c", cmdStr) // TAINT SINK: Command injection
	return cmd.Run()
}

// Restore restores from a backup
// VULN: Command injection via backup path
func (s *SQLiteStore) Restore(backupPath string) error {
	cmdStr := fmt.Sprintf("sqlite3 notifications.db '.restore %s'", backupPath)
	cmd := exec.Command("sh", "-c", cmdStr) // TAINT SINK
	return cmd.Run()
}

// Helper functions
func scanNotifications(rows *sql.Rows) ([]map[string]interface{}, error) {
	var results []map[string]interface{}

	cols, _ := rows.Columns()
	for rows.Next() {
		columns := make([]interface{}, len(cols))
		columnPtrs := make([]interface{}, len(cols))
		for i := range columns {
			columnPtrs[i] = &columns[i]
		}

		if err := rows.Scan(columnPtrs...); err != nil {
			continue
		}

		m := make(map[string]interface{})
		for i, colName := range cols {
			m[colName] = columns[i]
		}
		results = append(results, m)
	}

	return results, nil
}

func scanToMaps(rows *sql.Rows) ([]map[string]interface{}, error) {
	return scanNotifications(rows)
}

func metadataToJSON(metadata map[string]string) string {
	if metadata == nil {
		return "{}"
	}
	// Simple JSON encoding (not using encoding/json to show manual approach)
	pairs := make([]string, 0, len(metadata))
	for k, v := range metadata {
		pairs = append(pairs, fmt.Sprintf(`"%s":"%s"`, k, v))
	}
	return "{" + strings.Join(pairs, ",") + "}"
}

func generateAPIKey() string {
	// VULN: Weak API key generation
	return fmt.Sprintf("api_%d", time.Now().UnixNano())
}
