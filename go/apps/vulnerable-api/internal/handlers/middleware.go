package handlers

import (
	"fmt"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/go-chi/chi/v5"
	"github.com/gofiber/fiber/v2"
	"github.com/labstack/echo/v4"
)

// Middleware definitions for all frameworks
// These patterns are detected by go_http.py strategy for go_middleware table

// ===============================================
// GIN MIDDLEWARE
// ===============================================

// GinLoggingMiddleware logs all requests
func GinLoggingMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		start := time.Now()

		// Process request
		c.Next()

		// Log after request
		latency := time.Since(start)
		status := c.Writer.Status()
		method := c.Request.Method
		path := c.Request.URL.Path

		fmt.Printf("[GIN] %s %s - %d (%v)\n", method, path, status, latency)
	}
}

// GinAuthMiddleware checks authentication
func GinAuthMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		// TAINT SOURCE: Header value
		token := c.GetHeader("Authorization")

		if token == "" {
			c.AbortWithStatusJSON(http.StatusUnauthorized, gin.H{
				"error": "missing authorization",
			})
			return
		}

		// Set user in context (taint propagates through context)
		c.Set("user_token", token)
		c.Next()
	}
}

// GinRateLimitMiddleware applies rate limiting
func GinRateLimitMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		// Get client IP (potential taint source)
		clientIP := c.ClientIP()

		// Check rate limit (simplified)
		if isRateLimited(clientIP) {
			c.AbortWithStatusJSON(http.StatusTooManyRequests, gin.H{
				"error": "rate limit exceeded",
			})
			return
		}

		c.Next()
	}
}

// GinRecoveryMiddleware recovers from panics
func GinRecoveryMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		defer func() {
			if err := recover(); err != nil {
				c.AbortWithStatusJSON(http.StatusInternalServerError, gin.H{
					"error": "internal server error",
				})
			}
		}()
		c.Next()
	}
}

// ===============================================
// ECHO MIDDLEWARE
// ===============================================

// EchoLoggingMiddleware logs requests for Echo
func EchoLoggingMiddleware(next echo.HandlerFunc) echo.HandlerFunc {
	return func(c echo.Context) error {
		start := time.Now()

		// Process request
		err := next(c)

		// Log after
		latency := time.Since(start)
		status := c.Response().Status
		method := c.Request().Method
		path := c.Request().URL.Path

		fmt.Printf("[ECHO] %s %s - %d (%v)\n", method, path, status, latency)

		return err
	}
}

// EchoAuthMiddleware checks authentication for Echo
func EchoAuthMiddleware(next echo.HandlerFunc) echo.HandlerFunc {
	return func(c echo.Context) error {
		// TAINT SOURCE: Header value
		token := c.Request().Header.Get("Authorization")

		if token == "" {
			return c.JSON(http.StatusUnauthorized, map[string]string{
				"error": "missing authorization",
			})
		}

		// Set in context (taint propagation)
		c.Set("user_token", token)
		return next(c)
	}
}

// EchoCORSMiddleware handles CORS
func EchoCORSMiddleware(next echo.HandlerFunc) echo.HandlerFunc {
	return func(c echo.Context) error {
		// TAINT SOURCE: Origin header
		origin := c.Request().Header.Get("Origin")

		// Set CORS headers (potential reflection)
		c.Response().Header().Set("Access-Control-Allow-Origin", origin)
		c.Response().Header().Set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE")

		if c.Request().Method == "OPTIONS" {
			return c.NoContent(http.StatusOK)
		}

		return next(c)
	}
}

// ===============================================
// CHI MIDDLEWARE
// ===============================================

// ChiLoggingMiddleware logs requests for Chi
func ChiLoggingMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()

		// Wrap response writer to capture status
		wrapped := &responseWriter{ResponseWriter: w, status: 200}

		// Process request
		next.ServeHTTP(wrapped, r)

		// Log after
		latency := time.Since(start)
		fmt.Printf("[CHI] %s %s - %d (%v)\n", r.Method, r.URL.Path, wrapped.status, latency)
	})
}

// ChiAuthMiddleware checks authentication for Chi
func ChiAuthMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// TAINT SOURCE: Header value
		token := r.Header.Get("Authorization")

		if token == "" {
			http.Error(w, "missing authorization", http.StatusUnauthorized)
			return
		}

		// Add to context for downstream handlers
		ctx := r.Context()
		// In real code, would use context.WithValue
		_ = ctx

		next.ServeHTTP(w, r)
	})
}

// ChiUserContextMiddleware extracts user from URL
func ChiUserContextMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// TAINT SOURCE: URL parameter
		userID := chi.URLParam(r, "userID")

		if userID != "" {
			// User ID from URL flows into context
			fmt.Printf("Processing request for user: %s\n", userID)
		}

		next.ServeHTTP(w, r)
	})
}

// ===============================================
// FIBER MIDDLEWARE
// ===============================================

// FiberLoggingMiddleware logs requests for Fiber
func FiberLoggingMiddleware(c *fiber.Ctx) error {
	start := time.Now()

	// Process request
	err := c.Next()

	// Log after
	latency := time.Since(start)
	status := c.Response().StatusCode()
	method := c.Method()
	path := c.Path()

	fmt.Printf("[FIBER] %s %s - %d (%v)\n", method, path, status, latency)

	return err
}

// FiberAuthMiddleware checks authentication for Fiber
func FiberAuthMiddleware(c *fiber.Ctx) error {
	// TAINT SOURCE: Header value
	token := c.Get("Authorization")

	if token == "" {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
			"error": "missing authorization",
		})
	}

	// Store in locals (taint propagation)
	c.Locals("user_token", token)
	return c.Next()
}

// FiberRateLimitMiddleware applies rate limiting for Fiber
func FiberRateLimitMiddleware(c *fiber.Ctx) error {
	// Get client IP (potential taint source)
	clientIP := c.IP()

	// Check rate limit
	if isRateLimited(clientIP) {
		return c.Status(fiber.StatusTooManyRequests).JSON(fiber.Map{
			"error": "rate limit exceeded",
		})
	}

	return c.Next()
}

// FiberCORSMiddleware handles CORS for Fiber
func FiberCORSMiddleware(c *fiber.Ctx) error {
	// TAINT SOURCE: Origin header
	origin := c.Get("Origin")

	// Set CORS headers (potential reflection)
	c.Set("Access-Control-Allow-Origin", origin)
	c.Set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE")

	if c.Method() == "OPTIONS" {
		return c.SendStatus(fiber.StatusOK)
	}

	return c.Next()
}

// ===============================================
// HELPER TYPES AND FUNCTIONS
// ===============================================

// responseWriter wraps http.ResponseWriter to capture status
type responseWriter struct {
	http.ResponseWriter
	status int
}

func (w *responseWriter) WriteHeader(status int) {
	w.status = status
	w.ResponseWriter.WriteHeader(status)
}

// isRateLimited is a stub for rate limit checking
func isRateLimited(ip string) bool {
	// In real code, would check a rate limiter
	_ = ip
	return false
}
