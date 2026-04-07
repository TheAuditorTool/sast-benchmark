package models

import "time"

// User represents a system user
type User struct {
	ID        int64     `json:"id" orm:"auto"`
	Username  string    `json:"username" orm:"size(100)"`
	Email     string    `json:"email" orm:"size(100)"`
	Password  string    `json:"-" orm:"size(255)"`
	Role      string    `json:"role" orm:"size(50)"`
	APIKey    string    `json:"api_key,omitempty" orm:"size(64)"`
	CreatedAt time.Time `json:"created_at" orm:"auto_now_add;type(datetime)"`
	UpdatedAt time.Time `json:"updated_at" orm:"auto_now;type(datetime)"`
}

// TableName returns custom table name
func (u *User) TableName() string {
	return "users"
}

// AuditLog represents system audit entries
type AuditLog struct {
	ID        int64     `json:"id" orm:"auto"`
	UserID    int64     `json:"user_id"`
	Action    string    `json:"action" orm:"size(100)"`
	Resource  string    `json:"resource" orm:"size(100)"`
	Details   string    `json:"details" orm:"type(text)"`
	IPAddress string    `json:"ip_address" orm:"size(45)"`
	UserAgent string    `json:"user_agent" orm:"size(255)"`
	CreatedAt time.Time `json:"created_at" orm:"auto_now_add;type(datetime)"`
}

// Config represents system configuration
type Config struct {
	ID    int64  `json:"id" orm:"auto"`
	Key   string `json:"key" orm:"size(100);unique"`
	Value string `json:"value" orm:"type(text)"`
}

// Report represents generated reports
type Report struct {
	ID        int64     `json:"id" orm:"auto"`
	Name      string    `json:"name" orm:"size(100)"`
	Type      string    `json:"type" orm:"size(50)"`
	Query     string    `json:"query" orm:"type(text)"`
	Output    string    `json:"output" orm:"type(text)"`
	CreatedBy int64     `json:"created_by"`
	CreatedAt time.Time `json:"created_at" orm:"auto_now_add;type(datetime)"`
}
