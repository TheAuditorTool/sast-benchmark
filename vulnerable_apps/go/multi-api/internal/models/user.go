package models

import (
	"time"

	"gorm.io/gorm"
)

// User represents a user in the system
// This model is used by GORM for ORM operations
type User struct {
	gorm.Model
	ID        uint      `gorm:"primaryKey" json:"id"`
	Email     string    `gorm:"uniqueIndex;not null" json:"email"`
	Username  string    `gorm:"uniqueIndex;not null" json:"username"`
	Password  string    `gorm:"not null" json:"-"`
	Role      string    `gorm:"default:'user'" json:"role"`
	IsActive  bool      `gorm:"default:true" json:"is_active"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`

	// Relationships - GORM ORM edges
	Profile   *Profile  `gorm:"foreignKey:UserID" json:"profile,omitempty"`
	Posts     []Post    `gorm:"foreignKey:AuthorID" json:"posts,omitempty"`
	Comments  []Comment `gorm:"foreignKey:UserID" json:"comments,omitempty"`
	Sessions  []Session `gorm:"foreignKey:UserID" json:"sessions,omitempty"`
}

// Profile represents extended user information
type Profile struct {
	ID        uint   `gorm:"primaryKey" json:"id"`
	UserID    uint   `gorm:"uniqueIndex" json:"user_id"`
	FirstName string `json:"first_name"`
	LastName  string `json:"last_name"`
	Bio       string `gorm:"type:text" json:"bio"`
	AvatarURL string `json:"avatar_url"`
	Phone     string `json:"phone"`
	Address   string `json:"address"`

	// Back-reference
	User *User `gorm:"foreignKey:UserID" json:"user,omitempty"`
}

// Post represents a blog post
type Post struct {
	ID        uint      `gorm:"primaryKey" json:"id"`
	AuthorID  uint      `gorm:"index" json:"author_id"`
	Title     string    `gorm:"not null" json:"title"`
	Content   string    `gorm:"type:text" json:"content"`
	Slug      string    `gorm:"uniqueIndex" json:"slug"`
	Published bool      `gorm:"default:false" json:"published"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`

	// Relationships
	Author   *User     `gorm:"foreignKey:AuthorID" json:"author,omitempty"`
	Comments []Comment `gorm:"foreignKey:PostID" json:"comments,omitempty"`
	Tags     []Tag     `gorm:"many2many:post_tags" json:"tags,omitempty"`
}

// Comment represents a comment on a post
type Comment struct {
	ID        uint      `gorm:"primaryKey" json:"id"`
	PostID    uint      `gorm:"index" json:"post_id"`
	UserID    uint      `gorm:"index" json:"user_id"`
	Content   string    `gorm:"type:text;not null" json:"content"`
	CreatedAt time.Time `json:"created_at"`

	// Relationships
	Post *Post `gorm:"foreignKey:PostID" json:"post,omitempty"`
	User *User `gorm:"foreignKey:UserID" json:"user,omitempty"`
}

// Tag represents a post tag for categorization
type Tag struct {
	ID    uint   `gorm:"primaryKey" json:"id"`
	Name  string `gorm:"uniqueIndex;not null" json:"name"`
	Slug  string `gorm:"uniqueIndex;not null" json:"slug"`
	Posts []Post `gorm:"many2many:post_tags" json:"posts,omitempty"`
}

// Session represents an active user session
type Session struct {
	ID        uint      `gorm:"primaryKey" json:"id"`
	UserID    uint      `gorm:"index" json:"user_id"`
	Token     string    `gorm:"uniqueIndex;not null" json:"token"`
	UserAgent string    `json:"user_agent"`
	IPAddress string    `json:"ip_address"`
	ExpiresAt time.Time `json:"expires_at"`
	CreatedAt time.Time `json:"created_at"`

	// Relationship
	User *User `gorm:"foreignKey:UserID" json:"user,omitempty"`
}

// AuditLog tracks all security-relevant actions
type AuditLog struct {
	ID        uint      `gorm:"primaryKey" json:"id"`
	UserID    *uint     `gorm:"index" json:"user_id"`
	Action    string    `gorm:"not null" json:"action"`
	Resource  string    `json:"resource"`
	Details   string    `gorm:"type:text" json:"details"`
	IPAddress string    `json:"ip_address"`
	CreatedAt time.Time `json:"created_at"`
}

// FileUpload represents an uploaded file
type FileUpload struct {
	ID           uint      `gorm:"primaryKey" json:"id"`
	UserID       uint      `gorm:"index" json:"user_id"`
	OriginalName string    `json:"original_name"`
	StoredPath   string    `json:"stored_path"`
	MimeType     string    `json:"mime_type"`
	Size         int64     `json:"size"`
	CreatedAt    time.Time `json:"created_at"`

	// Relationship
	User *User `gorm:"foreignKey:UserID" json:"user,omitempty"`
}
