// Package api - Router setup for the notification service
package api

import (
	"database/sql"

	"github.com/gorilla/mux"
)

// NewRouter creates and configures the HTTP router
func NewRouter(h *Handlers, gh *GorillaHandlers, apiKey string) *mux.Router {
	r := mux.NewRouter()

	// Apply global middleware
	r.Use(LoggingMiddleware)
	r.Use(CORSMiddleware)
	r.Use(RecoveryMiddleware)

	// Public endpoints (no auth required)
	r.HandleFunc("/api/health", h.HealthCheck).Methods("GET")
	r.HandleFunc("/api/debug", h.DebugRequest).Methods("GET", "POST")

	// Callback endpoint (signature verified separately)
	r.HandleFunc("/api/callback/{id}", h.ProcessCallback).Methods("POST")

	// Protected API routes
	api := r.PathPrefix("/api").Subrouter()
	api.Use(APIKeyMiddleware(apiKey))

	// Notification endpoints
	api.HandleFunc("/notify", h.SendNotification).Methods("POST")
	api.HandleFunc("/notify/batch", h.SendBatchNotification).Methods("POST")
	api.HandleFunc("/notify/template", h.SendTemplatedNotification).Methods("POST")

	// Query endpoints
	api.HandleFunc("/notifications", h.ListNotifications).Methods("GET")
	api.HandleFunc("/notifications/search", h.SearchNotifications).Methods("GET")
	api.HandleFunc("/notifications/export", h.ExportNotifications).Methods("GET")

	// Webhook testing
	api.HandleFunc("/webhook/test", h.TestWebhook).Methods("POST")

	// Hook execution
	api.HandleFunc("/hooks/execute", h.ExecuteHook).Methods("POST")

	// Log access
	api.HandleFunc("/logs/{filename}", h.ReadLogFile).Methods("GET")

	// Config management
	api.HandleFunc("/config/import", h.ImportConfig).Methods("POST")

	// Proxy endpoint
	api.HandleFunc("/proxy", h.ProxyRequest).Methods("GET", "POST")

	// Cross-service endpoints (calls beego_admin)
	api.HandleFunc("/users/lookup", h.LookupUser).Methods("GET")
	api.HandleFunc("/notify/enriched", h.EnrichAndSendNotification).Methods("POST")

	// ===============================================
	// GORILLA MUX.VARS() TEST ROUTES
	// Demonstrates path parameter taint sources
	// ===============================================

	// User CRUD with mux.Vars
	api.HandleFunc("/users/{id}", gh.GetUser).Methods("GET")
	api.HandleFunc("/users/by-name/{username}", gh.GetUserByUsername).Methods("GET")
	api.HandleFunc("/users/by-email/{email}", gh.GetUserByEmail).Methods("GET")
	api.HandleFunc("/users/search/{category}/{status}", gh.SearchUsers).Methods("GET")
	api.HandleFunc("/users/{id}", gh.UpdateUser).Methods("PUT")
	api.HandleFunc("/users/{id}", gh.DeleteUser).Methods("DELETE")

	// Resource routes with multiple path params
	api.HandleFunc("/resources/{type}/{id}", gh.GetResource).Methods("GET")
	api.HandleFunc("/resources/{type}/{id}", gh.UpdateResource).Methods("PUT")

	// Nested resources (deep path params)
	api.HandleFunc("/users/{user_id}/posts/{post_id}/comments/{comment_id}", gh.GetNestedResource).Methods("GET")

	// Wildcard path (path traversal testing)
	api.HandleFunc("/files/{path:.*}", gh.ServeFile).Methods("GET")

	// Command injection via path params
	api.HandleFunc("/tools/{tool}/run", gh.RunTool).Methods("POST")
	api.HandleFunc("/network/ping/{host}", gh.PingHost).Methods("GET")

	// Path + header sources
	api.HandleFunc("/audit/{action}", gh.AuditAction).Methods("POST")

	// Path + cookie sources
	api.HandleFunc("/sessions/{session_id}", gh.GetUserSession).Methods("GET")

	// Multi-hop with path params
	api.HandleFunc("/users/{id}/process", gh.ProcessUserData).Methods("POST")

	// Regex path params
	api.HandleFunc("/users/pattern/{pattern:[a-z]+}", gh.GetUserByPattern).Methods("GET")

	// Secure endpoints for comparison
	api.HandleFunc("/users/secure/{id}", gh.GetUserSecure).Methods("GET")
	api.HandleFunc("/users/secure/{id}", gh.UpdateUserSecure).Methods("PUT")

	return r
}

// NewRouterWithDB creates router with database connection for GorillaHandlers
func NewRouterWithDB(h *Handlers, db *sql.DB, apiKey string) *mux.Router {
	gh := NewGorillaHandlers(db)
	return NewRouter(h, gh, apiKey)
}
