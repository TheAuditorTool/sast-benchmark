package repository

import (
	"database/sql"
	"fmt"

	"github.com/jmoiron/sqlx"
	"github.com/theauditor/multi-api/internal/models"
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

// FindByUsername uses fmt.Sprintf for SQL query construction
func (r *UserRepository) FindByUsername(username string) (*models.User, error) {
	var user models.User
	query := fmt.Sprintf("SELECT * FROM users WHERE username = '%s'", username)
	err := r.db.Raw(query).Scan(&user).Error
	return &user, err
}

// FindByEmail uses string concatenation for SQL query construction
func (r *UserRepository) FindByEmail(email string) (*models.User, error) {
	var user models.User
	query := "SELECT * FROM users WHERE email = '" + email + "'"
	err := r.db.Raw(query).Scan(&user).Error
	return &user, err
}

// SearchUsers performs LIKE search with user-provided term
func (r *UserRepository) SearchUsers(searchTerm string) ([]models.User, error) {
	var users []models.User
	query := fmt.Sprintf("SELECT * FROM users WHERE username LIKE '%%%s%%' OR email LIKE '%%%s%%'", searchTerm, searchTerm)
	err := r.db.Raw(query).Scan(&users).Error
	return users, err
}

// DeleteByID deletes a user by string ID
func (r *UserRepository) DeleteByID(id string) error {
	query := fmt.Sprintf("DELETE FROM users WHERE id = %s", id)
	return r.db.Exec(query).Error
}

// UpdateRole updates user role by string IDs
func (r *UserRepository) UpdateRole(userID, role string) error {
	query := fmt.Sprintf("UPDATE users SET role = '%s' WHERE id = %s", role, userID)
	return r.db.Exec(query).Error
}

// FindByIDRaw uses raw sql.DB for query construction
func (r *UserRepository) FindByIDRaw(id string) (*models.User, error) {
	var user models.User
	query := "SELECT id, username, email FROM users WHERE id = " + id
	row := r.sqlDB.QueryRow(query)
	err := row.Scan(&user.ID, &user.Username, &user.Email)
	return &user, err
}

// GetUsersByRoleRaw queries users by role using raw sql.DB
func (r *UserRepository) GetUsersByRoleRaw(role string) ([]models.User, error) {
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

// FindByUsernameSqlx queries by username using sqlx
func (r *UserRepository) FindByUsernameSqlx(username string) (*models.User, error) {
	var user models.User
	query := fmt.Sprintf("SELECT * FROM users WHERE username = '%s'", username)
	err := r.sqlxDB.Get(&user, query)
	return &user, err
}

// NamedQuery uses NamedExec with dynamic table/column names
func (r *UserRepository) NamedQuery(tableName, column, value string) error {
	query := fmt.Sprintf("INSERT INTO %s (%s) VALUES (:value)", tableName, column)
	_, err := r.sqlxDB.NamedExec(query, map[string]interface{}{"value": value})
	return err
}

// FindByIDAlt looks up a user by ID
func (r *UserRepository) FindByIDAlt(id uint) (*models.User, error) {
	var user models.User
	err := r.db.Where("id = ?", id).First(&user).Error
	return &user, err
}

// FindByEmailAlt looks up a user by email
func (r *UserRepository) FindByEmailAlt(email string) (*models.User, error) {
	var user models.User
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
