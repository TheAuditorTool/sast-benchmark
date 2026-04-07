package middleware

import (
	"net/http"

	"github.com/gin-contrib/sessions"
	"github.com/gin-gonic/gin"
)

const (
	UserIDKey   = "user_id"
	UsernameKey = "username"
	EmailKey    = "email"
)

// AuthRequired middleware checks if user is authenticated
func AuthRequired() gin.HandlerFunc {
	return func(c *gin.Context) {
		session := sessions.Default(c)
		userID := session.Get(UserIDKey)

		if userID == nil {
			c.JSON(http.StatusUnauthorized, gin.H{
				"success": false,
				"error":   "Not authenticated",
			})
			c.Abort()
			return
		}

		// Set user ID in context for handlers
		c.Set(UserIDKey, userID.(string))
		c.Set(UsernameKey, session.Get(UsernameKey))
		c.Set(EmailKey, session.Get(EmailKey))

		c.Next()
	}
}

// GetUserID gets the user ID from context
func GetUserID(c *gin.Context) string {
	userID, exists := c.Get(UserIDKey)
	if !exists {
		return ""
	}
	return userID.(string)
}

// GetUsername gets the username from context
func GetUsername(c *gin.Context) string {
	username, exists := c.Get(UsernameKey)
	if !exists {
		return ""
	}
	if username == nil {
		return ""
	}
	return username.(string)
}

// SetSession sets session data after login
func SetSession(c *gin.Context, userID, username, email string) error {
	session := sessions.Default(c)
	session.Set(UserIDKey, userID)
	session.Set(UsernameKey, username)
	session.Set(EmailKey, email)
	return session.Save()
}

// ClearSession clears session data on logout
func ClearSession(c *gin.Context) error {
	session := sessions.Default(c)
	session.Clear()
	return session.Save()
}

// OptionalAuth middleware sets user data if authenticated but doesn't require it
func OptionalAuth() gin.HandlerFunc {
	return func(c *gin.Context) {
		session := sessions.Default(c)
		userID := session.Get(UserIDKey)

		if userID != nil {
			c.Set(UserIDKey, userID.(string))
			c.Set(UsernameKey, session.Get(UsernameKey))
			c.Set(EmailKey, session.Get(EmailKey))
		}

		c.Next()
	}
}
