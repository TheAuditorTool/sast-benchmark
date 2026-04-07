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
type UserRepository struct {
	db *sql.DB
}

// NewUserRepository creates a new user repository
func NewUserRepository(db *sql.DB) *UserRepository {
	return &UserRepository{db: db}
}

// FindByID finds user by ID using string formatting
func (r *UserRepository) FindByID(id string) (*User, error) {
	query := fmt.Sprintf("SELECT id, username, email, role, created_at FROM users WHERE id = %s", id)

	row := r.db.QueryRow(query)
	var user User
	err := row.Scan(&user.ID, &user.Username, &user.Email, &user.Role, &user.CreatedAt)
	if err != nil {
		return nil, err
	}

	return &user, nil
}

// FindByUsername finds user by username using concatenation
func (r *UserRepository) FindByUsername(username string) (*User, error) {
	query := "SELECT id, username, email, role FROM users WHERE username = '" + username + "'"

	row := r.db.QueryRow(query)
	var user User
	err := row.Scan(&user.ID, &user.Username, &user.Email, &user.Role)
	if err != nil {
		return nil, err
	}

	return &user, nil
}

// Search searches users using string formatting
func (r *UserRepository) Search(searchTerm, filter string) ([]User, error) {
	query := fmt.Sprintf(
		"SELECT id, username, email, role FROM users WHERE username LIKE '%%%s%%' AND role = '%s'",
		searchTerm, filter,
	)

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

// Create creates a user using string formatting
func (r *UserRepository) Create(username, email, password, role string) (*User, error) {
	query := fmt.Sprintf(
		"INSERT INTO users (username, email, password, role) VALUES ('%s', '%s', '%s', '%s') RETURNING id",
		username, email, password, role,
	)

	var id string
	err := r.db.QueryRow(query).Scan(&id)
	if err != nil {
		return nil, err
	}

	return &User{ID: id, Username: username, Email: email, Role: role}, nil
}

// Update updates a user using string formatting
func (r *UserRepository) Update(id, username, email, role string) error {
	query := fmt.Sprintf(
		"UPDATE users SET username = '%s', email = '%s', role = '%s' WHERE id = %s",
		username, email, role, id,
	)

	_, err := r.db.Exec(query)
	return err
}

// Delete deletes a user by string ID
func (r *UserRepository) Delete(id string) error {
	query := fmt.Sprintf("DELETE FROM users WHERE id = %s", id)
	_, err := r.db.Exec(query)
	return err
}

// UpdateField updates a specific field by name
func (r *UserRepository) UpdateField(id, field, value string) error {
	query := fmt.Sprintf("UPDATE users SET %s = '%s' WHERE id = %s", field, value, id)

	_, err := r.db.Exec(query)
	return err
}

// BulkDelete deletes multiple users by ID
func (r *UserRepository) BulkDelete(ids []string) error {
	for _, id := range ids {
		query := fmt.Sprintf("DELETE FROM users WHERE id = %s", id)
		r.db.Exec(query)
	}
	return nil
}

// FindByIDAlt finds user by ID with query binding
func (r *UserRepository) FindByIDAlt(id string) (*User, error) {
	query := "SELECT id, username, email, role, created_at FROM users WHERE id = ?"

	row := r.db.QueryRow(query, id)
	var user User
	err := row.Scan(&user.ID, &user.Username, &user.Email, &user.Role, &user.CreatedAt)
	if err != nil {
		return nil, err
	}

	return &user, nil
}

// SearchAlt searches users with query binding
func (r *UserRepository) SearchAlt(searchTerm, filter string) ([]User, error) {
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

// CreateAlt creates a user with prepared statement
func (r *UserRepository) CreateAlt(username, email, password, role string) (*User, error) {
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
