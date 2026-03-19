package repository

import (
	"database/sql"
	"fmt"
)

// User represents a user entity
type User struct {
	ID        string
	Username  string
	Email     string
	Password  string
	Role      string
	CreatedAt string
}

// UserRepository handles database operations for users
// MULTI-HOP LAYER: Receives tainted input from server, executes SQL
type UserRepository struct {
	db *sql.DB
}

// NewUserRepository creates a new user repository
func NewUserRepository(db *sql.DB) *UserRepository {
	return &UserRepository{db: db}
}

// ===============================================
// TAINT HOP 2: Repository Layer
// Receives tainted input from gRPC server
// ===============================================

// FindByIDVulnerable finds user by ID - VULNERABLE
// TAINT HOP 2: Receives tainted ID from server
// TAINT SINK: SQL injection
func (r *UserRepository) FindByIDVulnerable(id string) (*User, error) {
	// TAINT PROPAGATION: Parameter to SQL
	query := fmt.Sprintf("SELECT id, username, email, role, created_at FROM users WHERE id = %s", id)

	// TAINT SINK: SQL injection
	row := r.db.QueryRow(query)
	var user User
	err := row.Scan(&user.ID, &user.Username, &user.Email, &user.Role, &user.CreatedAt)
	if err != nil {
		return nil, err
	}

	return &user, nil
}

// FindByUsernameVulnerable finds user by username - VULNERABLE
// TAINT HOP 2: Receives tainted username
func (r *UserRepository) FindByUsernameVulnerable(username string) (*User, error) {
	// TAINT PROPAGATION: String concatenation
	query := "SELECT id, username, email, role FROM users WHERE username = '" + username + "'"

	// TAINT SINK: SQL injection
	row := r.db.QueryRow(query)
	var user User
	err := row.Scan(&user.ID, &user.Username, &user.Email, &user.Role)
	if err != nil {
		return nil, err
	}

	return &user, nil
}

// SearchVulnerable searches users - VULNERABLE
// TAINT HOP 2: Receives tainted searchTerm and filter
func (r *UserRepository) SearchVulnerable(searchTerm, filter string) ([]User, error) {
	// TAINT PROPAGATION: Both parameters in query
	query := fmt.Sprintf(
		"SELECT id, username, email, role FROM users WHERE username LIKE '%%%s%%' AND role = '%s'",
		searchTerm, filter,
	)

	// TAINT SINK: SQL injection
	rows, err := r.db.Query(query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var users []User
	for rows.Next() {
		var user User
		if err := rows.Scan(&user.ID, &user.Username, &user.Email, &user.Role); err != nil {
			continue
		}
		users = append(users, user)
	}

	return users, nil
}

// CreateVulnerable creates a user - VULNERABLE
// TAINT HOP 2: Receives all tainted fields
func (r *UserRepository) CreateVulnerable(username, email, password, role string) (*User, error) {
	// TAINT PROPAGATION: All parameters in INSERT
	query := fmt.Sprintf(
		"INSERT INTO users (username, email, password, role) VALUES ('%s', '%s', '%s', '%s') RETURNING id",
		username, email, password, role,
	)

	// TAINT SINK: SQL injection
	var id string
	err := r.db.QueryRow(query).Scan(&id)
	if err != nil {
		return nil, err
	}

	return &User{ID: id, Username: username, Email: email, Role: role}, nil
}

// UpdateVulnerable updates a user - VULNERABLE
// TAINT HOP 2: All parameters tainted
func (r *UserRepository) UpdateVulnerable(id, username, email, role string) error {
	// TAINT PROPAGATION: All parameters in UPDATE
	query := fmt.Sprintf(
		"UPDATE users SET username = '%s', email = '%s', role = '%s' WHERE id = %s",
		username, email, role, id,
	)

	// TAINT SINK: SQL injection
	_, err := r.db.Exec(query)
	return err
}

// DeleteVulnerable deletes a user - VULNERABLE
// TAINT HOP 2: ID is tainted
func (r *UserRepository) DeleteVulnerable(id string) error {
	// TAINT PROPAGATION -> SINK
	query := fmt.Sprintf("DELETE FROM users WHERE id = %s", id)
	_, err := r.db.Exec(query)
	return err
}

// UpdateFieldVulnerable updates a specific field - VULNERABLE
// TAINT HOP 2: All parameters including field name
func (r *UserRepository) UpdateFieldVulnerable(id, field, value string) error {
	// TAINT PROPAGATION: Column name injection possible
	query := fmt.Sprintf("UPDATE users SET %s = '%s' WHERE id = %s", field, value, id)

	// TAINT SINK: SQL injection
	_, err := r.db.Exec(query)
	return err
}

// BulkDeleteVulnerable deletes multiple users - VULNERABLE
// TAINT HOP 2: IDs array is tainted
func (r *UserRepository) BulkDeleteVulnerable(ids []string) error {
	// TAINT PROPAGATION: Join IDs directly
	for _, id := range ids {
		query := fmt.Sprintf("DELETE FROM users WHERE id = %s", id)
		// TAINT SINK: SQL injection for each ID
		r.db.Exec(query)
	}
	return nil
}

// ===============================================
// SECURE EXAMPLES (for comparison)
// ===============================================

// FindByIDSecure finds user by ID - SECURE
func (r *UserRepository) FindByIDSecure(id string) (*User, error) {
	// SECURE: Parameterized query
	query := "SELECT id, username, email, role, created_at FROM users WHERE id = ?"

	row := r.db.QueryRow(query, id)
	var user User
	err := row.Scan(&user.ID, &user.Username, &user.Email, &user.Role, &user.CreatedAt)
	if err != nil {
		return nil, err
	}

	return &user, nil
}

// SearchSecure searches users - SECURE
func (r *UserRepository) SearchSecure(searchTerm, filter string) ([]User, error) {
	// SECURE: Parameterized query
	query := "SELECT id, username, email, role FROM users WHERE username LIKE ? AND role = ?"

	rows, err := r.db.Query(query, "%"+searchTerm+"%", filter)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var users []User
	for rows.Next() {
		var user User
		if err := rows.Scan(&user.ID, &user.Username, &user.Email, &user.Role); err != nil {
			continue
		}
		users = append(users, user)
	}

	return users, nil
}

// CreateSecure creates a user - SECURE
func (r *UserRepository) CreateSecure(username, email, password, role string) (*User, error) {
	// SECURE: Prepared statement
	stmt, err := r.db.Prepare("INSERT INTO users (username, email, password, role) VALUES (?, ?, ?, ?)")
	if err != nil {
		return nil, err
	}
	defer stmt.Close()

	result, err := stmt.Exec(username, email, password, role)
	if err != nil {
		return nil, err
	}

	id, _ := result.LastInsertId()
	return &User{ID: fmt.Sprintf("%d", id), Username: username, Email: email, Role: role}, nil
}
