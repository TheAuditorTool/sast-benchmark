package repository

import (
	"database/sql"
	"fmt"

	"github.com/jmoiron/sqlx"
	"github.com/theauditor/vulnerable-api/internal/models"
	"gorm.io/gorm"
)

// UserRepository handles user database operations
type UserRepository struct {
	db    *gorm.DB
	sqlDB *sql.DB
	sqlxDB *sqlx.DB
}

// NewUserRepository creates a new user repository
func NewUserRepository(db *gorm.DB, sqlDB *sql.DB, sqlxDB *sqlx.DB) *UserRepository {
	return &UserRepository{
		db:    db,
		sqlDB: sqlDB,
		sqlxDB: sqlxDB,
	}
}

// ===============================================
// VULNERABLE: SQL Injection via string formatting
// ===============================================

// FindByUsername - VULNERABLE: SQL injection via fmt.Sprintf
func (r *UserRepository) FindByUsername(username string) (*models.User, error) {
	var user models.User
	// VULNERABILITY: SQL injection - user input directly in query
	query := fmt.Sprintf("SELECT * FROM users WHERE username = '%s'", username)
	err := r.db.Raw(query).Scan(&user).Error
	return &user, err
}

// FindByEmail - VULNERABLE: SQL injection via string concatenation
func (r *UserRepository) FindByEmail(email string) (*models.User, error) {
	var user models.User
	// VULNERABILITY: SQL injection - string concatenation
	query := "SELECT * FROM users WHERE email = '" + email + "'"
	err := r.db.Raw(query).Scan(&user).Error
	return &user, err
}

// SearchUsers - VULNERABLE: SQL injection in LIKE clause
func (r *UserRepository) SearchUsers(searchTerm string) ([]models.User, error) {
	var users []models.User
	// VULNERABILITY: SQL injection - search term not escaped
	query := fmt.Sprintf("SELECT * FROM users WHERE username LIKE '%%%s%%' OR email LIKE '%%%s%%'", searchTerm, searchTerm)
	err := r.db.Raw(query).Scan(&users).Error
	return users, err
}

// DeleteByID - VULNERABLE: SQL injection in DELETE
func (r *UserRepository) DeleteByID(id string) error {
	// VULNERABILITY: SQL injection - id not validated
	query := fmt.Sprintf("DELETE FROM users WHERE id = %s", id)
	return r.db.Exec(query).Error
}

// UpdateRole - VULNERABLE: SQL injection via UPDATE
func (r *UserRepository) UpdateRole(userID, role string) error {
	// VULNERABILITY: SQL injection - both parameters vulnerable
	query := fmt.Sprintf("UPDATE users SET role = '%s' WHERE id = %s", role, userID)
	return r.db.Exec(query).Error
}

// ===============================================
// VULNERABLE: Using database/sql directly
// ===============================================

// FindByIDRaw - VULNERABLE: SQL injection with raw sql.DB
func (r *UserRepository) FindByIDRaw(id string) (*models.User, error) {
	var user models.User
	// VULNERABILITY: SQL injection - using raw database/sql
	query := "SELECT id, username, email FROM users WHERE id = " + id
	row := r.sqlDB.QueryRow(query)
	err := row.Scan(&user.ID, &user.Username, &user.Email)
	return &user, err
}

// GetUsersByRoleRaw - VULNERABLE: SQL injection with Query
func (r *UserRepository) GetUsersByRoleRaw(role string) ([]models.User, error) {
	// VULNERABILITY: SQL injection via Query
	query := fmt.Sprintf("SELECT * FROM users WHERE role = '%s'", role)
	rows, err := r.sqlDB.Query(query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var users []models.User
	for rows.Next() {
		var user models.User
		if err := rows.Scan(&user.ID, &user.Username, &user.Email); err != nil {
			return nil, err
		}
		users = append(users, user)
	}
	return users, nil
}

// ===============================================
// VULNERABLE: Using sqlx
// ===============================================

// FindByUsernameSqlx - VULNERABLE: SQL injection with sqlx
func (r *UserRepository) FindByUsernameSqlx(username string) (*models.User, error) {
	var user models.User
	// VULNERABILITY: SQL injection via sqlx.Get
	query := fmt.Sprintf("SELECT * FROM users WHERE username = '%s'", username)
	err := r.sqlxDB.Get(&user, query)
	return &user, err
}

// NamedQueryVulnerable - VULNERABLE: SQL injection with NamedExec
func (r *UserRepository) NamedQueryVulnerable(tableName, column, value string) error {
	// VULNERABILITY: Table and column names from user input
	query := fmt.Sprintf("INSERT INTO %s (%s) VALUES (:value)", tableName, column)
	_, err := r.sqlxDB.NamedExec(query, map[string]interface{}{"value": value})
	return err
}

// ===============================================
// SECURE: Parameterized queries (for comparison)
// ===============================================

// FindByIDSecure - SECURE: Uses parameterized query
func (r *UserRepository) FindByIDSecure(id uint) (*models.User, error) {
	var user models.User
	// SECURE: Parameterized query
	err := r.db.Where("id = ?", id).First(&user).Error
	return &user, err
}

// FindByEmailSecure - SECURE: Uses GORM's built-in protection
func (r *UserRepository) FindByEmailSecure(email string) (*models.User, error) {
	var user models.User
	// SECURE: GORM handles parameterization
	err := r.db.Where(&models.User{Email: email}).First(&user).Error
	return &user, err
}

// CreateUser - Uses GORM Create (safe by default)
func (r *UserRepository) CreateUser(user *models.User) error {
	return r.db.Create(user).Error
}

// UpdateUser - Uses GORM Save (safe by default)
func (r *UserRepository) UpdateUser(user *models.User) error {
	return r.db.Save(user).Error
}

// DeleteUser - Uses GORM Delete (safe by default)
func (r *UserRepository) DeleteUser(id uint) error {
	return r.db.Delete(&models.User{}, id).Error
}
