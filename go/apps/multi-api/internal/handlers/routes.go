package handlers

import (
	"database/sql"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/go-chi/chi/v5"
	"github.com/gofiber/fiber/v2"
	"github.com/labstack/echo/v4"
	"github.com/theauditor/multi-api/internal/repository"
)

// RouteRegistrar handles route registration for all frameworks
// This file demonstrates different routing patterns for TheAuditor's go_routes extraction

// SetupGinRoutes configures Gin routes
// These patterns are detected by go_http.py strategy
func SetupGinRoutes(r *gin.Engine, userRepo *repository.UserRepository, db *sql.DB) {
	h := NewGinHandler(userRepo, db)

	// API group with versioning
	v1 := r.Group("/api/v1")
	{
		// User routes - GET handlers
		v1.GET("/users", h.GetUser)
		v1.GET("/users/:id", h.GetUserAlt)
		v1.GET("/users/by-username/:username", h.GetUserByUsername)

		// User routes - POST/PUT/DELETE
		v1.POST("/users", h.CreateUser)
		v1.POST("/users/search", h.SearchUsers)

		// Command execution (vulnerable)
		v1.GET("/cmd", h.RunCommand)
		v1.GET("/ping/:host", h.PingHost)

		// Template rendering (XSS)
		v1.GET("/profile", h.RenderProfile)
		v1.GET("/script", h.RenderJS)

		// File operations (path traversal)
		v1.GET("/download/:filename", h.DownloadFile)
		v1.GET("/config", h.ReadConfig)
		v1.POST("/save", h.SaveFile)

		// Complex taint flow
		v1.POST("/complex", h.ComplexFlow)
	}

	// Admin routes with middleware
	admin := r.Group("/admin")
	admin.Use(func(c *gin.Context) {
		// Middleware that might add context
		c.Set("admin", true)
		c.Next()
	})
	{
		admin.GET("/users/secure", h.GetUserAlt)
		admin.GET("/cmd/secure", h.RunCommandAlt)
	}
}

// SetupEchoRoutes configures Echo routes
func SetupEchoRoutes(e *echo.Echo, userRepo *repository.UserRepository, db *sql.DB) {
	h := NewEchoHandler(userRepo, db)

	// API group
	api := e.Group("/api")
	{
		// User routes
		api.GET("/users", h.GetUser)
		api.GET("/users/:username", h.GetUserByUsername)
		api.POST("/users/search", h.SearchUsers)
		api.POST("/users", h.CreateUser)
		api.PUT("/users/:id", h.UpdateUser)

		// Diagnostic (command injection)
		api.GET("/diagnostic", h.RunDiagnostic)
		api.POST("/shell", h.ExecShell)

		// File operations
		api.GET("/files", h.ServeFile)
		api.DELETE("/files/:filename", h.DeleteFile)

		// Request processing
		api.POST("/process", h.ProcessRequest)

		// Reports with cross-function taint
		api.GET("/reports", h.ReportEndpoint)
	}

	// Alt endpoints
	secure := e.Group("/secure")
	{
		secure.GET("/users", h.GetUserAlt)
	}
}

// SetupChiRoutes configures Chi routes
func SetupChiRoutes(r chi.Router, userRepo *repository.UserRepository, db *sql.DB) {
	h := NewChiHandler(userRepo, db)

	// Nested routing
	r.Route("/api", func(r chi.Router) {
		r.Route("/users", func(r chi.Router) {
			r.Get("/", h.SearchUsers)
			r.Get("/{id}", h.GetUser)
			r.Get("/by-username/{username}", h.GetUserByUsername)
			r.Post("/", h.CreateUser)
			r.Put("/{id}", h.UpdateUser)
			r.Delete("/{id}", h.DeleteUser)

			// Alt sub-routes
			r.Get("/secure/{id}", h.GetUserAlt)
			r.Post("/secure", h.CreateUserAlt)
		})

		r.Route("/commands", func(r chi.Router) {
			r.Get("/", h.RunCommand)
			r.Get("/sysinfo", h.SystemInfo)
		})

		r.Route("/files", func(r chi.Router) {
			r.Get("/*", h.ServeFile)
			r.Post("/upload", h.UploadFile)
		})

		r.Post("/audit/{action}", h.AuditLog)
	})
}

// SetupFiberRoutes configures Fiber routes
func SetupFiberRoutes(app *fiber.App, userRepo *repository.UserRepository, db *sql.DB) {
	h := NewFiberHandler(userRepo, db)

	// API group
	api := app.Group("/api")
	{
		// User routes
		api.Get("/users", h.GetUser)
		api.Get("/users/:username", h.GetUserByUsername)
		api.Post("/users/search", h.SearchUsers)
		api.Post("/users", h.CreateUser)
		api.Put("/users/:id", h.UpdateUser)
		api.Delete("/users/:id", h.DeleteUser)

		// Command execution
		api.Get("/cmd", h.RunCommand)
		api.Post("/shell", h.ExecShell)

		// File operations
		api.Get("/files/*", h.ServeFile)
		api.Get("/download", h.DownloadFile)

		// Header/Cookie processing
		api.Post("/headers", h.ProcessHeaders)
		api.Get("/cookies", h.ProcessCookies)

		// Batch operations
		api.Post("/batch", h.BatchOperation)
	}

	// Alt endpoints
	secure := api.Group("/secure")
	{
		secure.Get("/users", h.GetUserAlt)
		secure.Post("/users", h.CreateUserAlt)
	}
}

// SetupNetHTTPRoutes configures standard net/http routes
func SetupNetHTTPRoutes(mux *http.ServeMux, userRepo *repository.UserRepository, db *sql.DB) {
	h := NewNetHTTPHandler(userRepo, db)

	// User routes
	mux.HandleFunc("/api/users", h.GetUser)
	mux.HandleFunc("/api/users/search", h.SearchUsers)
	mux.HandleFunc("/api/users/create", h.CreateUser)
	mux.HandleFunc("/api/users/update", h.UpdateUser)

	// JSON processing
	mux.HandleFunc("/api/process", h.ProcessJSON)
	mux.HandleFunc("/api/raw", h.RawBodyToSQL)

	// Command execution
	mux.HandleFunc("/api/cmd", h.RunCommand)
	mux.HandleFunc("/api/shell", h.ShellExec)
	mux.HandleFunc("/api/process-file", h.ProcessFile)

	// File operations
	mux.HandleFunc("/api/files", h.ServeFile)
	mux.HandleFunc("/api/files/", h.ReadFile)
	mux.HandleFunc("/api/write", h.WriteFile)

	// Header/Cookie logging
	mux.HandleFunc("/api/log", h.LogRequest)
	mux.HandleFunc("/api/cookie", h.ProcessCookie)

	// Complex taint flow
	mux.HandleFunc("/api/complex", h.ComplexHandler)

	// Alt endpoints
	mux.HandleFunc("/api/users/secure", h.GetUserAlt)
	mux.HandleFunc("/api/users/secure/create", h.CreateUserAlt)
}
