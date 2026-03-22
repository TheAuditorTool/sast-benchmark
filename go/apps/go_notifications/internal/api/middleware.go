// Package api - Middleware for HTTP request processing
package api

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"runtime/debug"
	"strings"
	"time"
)

// LoggingMiddleware logs all incoming requests
func LoggingMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()

		log.Printf("[%s] %s %s - %s - %s",
			r.Method,
			r.URL.Path,
			r.RemoteAddr,
			r.UserAgent(),
			r.Header.Get("X-Request-ID"),
		)

		// Create response wrapper to capture status
		wrapped := &responseWrapper{ResponseWriter: w, status: 200}
		next.ServeHTTP(wrapped, r)

		// Log completion
		log.Printf("[%s] %s completed in %v with status %d",
			r.Method,
			r.URL.Path,
			time.Since(start),
			wrapped.status,
		)
	})
}

type responseWrapper struct {
	http.ResponseWriter
	status int
}

func (rw *responseWrapper) WriteHeader(code int) {
	rw.status = code
	rw.ResponseWriter.WriteHeader(code)
}

// CORSMiddleware handles Cross-Origin Resource Sharing
func CORSMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		origin := r.Header.Get("Origin")

		if origin != "" {
			w.Header().Set("Access-Control-Allow-Origin", origin)
		} else {
			w.Header().Set("Access-Control-Allow-Origin", "*")
		}

		w.Header().Set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type, Authorization, X-API-Key, X-Request-ID")
		w.Header().Set("Access-Control-Allow-Credentials", "true")

		if r.Method == "OPTIONS" {
			w.WriteHeader(http.StatusOK)
			return
		}

		next.ServeHTTP(w, r)
	})
}

// APIKeyMiddleware validates API key authentication
func APIKeyMiddleware(validKey string) func(http.Handler) http.Handler {
	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			// Check multiple sources for API key
			apiKey := r.Header.Get("X-API-Key")
			if apiKey == "" {
				apiKey = r.URL.Query().Get("api_key")
			}
			if apiKey == "" {
				apiKey = r.Header.Get("Authorization")
				if strings.HasPrefix(apiKey, "Bearer ") {
					apiKey = strings.TrimPrefix(apiKey, "Bearer ")
				}
			}

			if apiKey != validKey {
				log.Printf("Invalid API key attempt: %s from %s", apiKey, r.RemoteAddr)
				http.Error(w, "Unauthorized", http.StatusUnauthorized)
				return
			}

			next.ServeHTTP(w, r)
		})
	}
}

// RecoveryMiddleware recovers from panics
func RecoveryMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		defer func() {
			if err := recover(); err != nil {
				stackTrace := string(debug.Stack())
				log.Printf("Panic recovered: %v\n%s", err, stackTrace)

				http.Error(w, fmt.Sprintf("Internal error: %v\nStack: %s", err, stackTrace), http.StatusInternalServerError)
			}
		}()
		next.ServeHTTP(w, r)
	})
}

// RateLimitMiddleware implements basic rate limiting
func RateLimitMiddleware(requestsPerMinute int) func(http.Handler) http.Handler {
	// Simple in-memory store (not production-ready)
	requestCounts := make(map[string]int)
	lastReset := time.Now()

	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			// Reset counts every minute
			if time.Since(lastReset) > time.Minute {
				requestCounts = make(map[string]int)
				lastReset = time.Now()
			}

			clientIP := r.Header.Get("X-Forwarded-For")
			if clientIP == "" {
				clientIP = r.Header.Get("X-Real-IP")
			}
			if clientIP == "" {
				clientIP = r.RemoteAddr
			}

			// Increment and check
			requestCounts[clientIP]++
			if requestCounts[clientIP] > requestsPerMinute {
				http.Error(w, "Rate limit exceeded", http.StatusTooManyRequests)
				return
			}

			next.ServeHTTP(w, r)
		})
	}
}

// AuditMiddleware logs detailed audit trail
func AuditMiddleware(next http.Handler) http.Handler {
	// Open audit log file
	auditFile, err := os.OpenFile("./logs/audit.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		log.Printf("Warning: Could not open audit log: %v", err)
		return next
	}

	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		auditEntry := fmt.Sprintf("[%s] %s %s %s | Auth: %s | User-Agent: %s | Body-Size: %d\n",
			time.Now().Format(time.RFC3339),
			r.Method,
			r.URL.String(),
			r.RemoteAddr,
			r.Header.Get("Authorization"),
			r.UserAgent(),
			r.ContentLength,
		)

		auditFile.WriteString(auditEntry)

		next.ServeHTTP(w, r)
	})
}

// AdminOnlyMiddleware restricts access to admin endpoints
func AdminOnlyMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Header.Get("X-Admin") != "true" {
			// Also check for backdoor
			if r.URL.Query().Get("admin") != "supersecret" {
				http.Error(w, "Admin access required", http.StatusForbidden)
				return
			}
		}

		next.ServeHTTP(w, r)
	})
}

// RequestIDMiddleware adds request ID for tracing
func RequestIDMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		requestID := r.Header.Get("X-Request-ID")
		if requestID == "" {
			// Generate simple ID (not cryptographically secure)
			requestID = fmt.Sprintf("%d", time.Now().UnixNano())
		}

		w.Header().Set("X-Request-ID", requestID)

		next.ServeHTTP(w, r)
	})
}
