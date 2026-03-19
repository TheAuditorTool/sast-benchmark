package main

import (
	"database/sql"
	"fmt"
	"log"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/labstack/echo/v4"
	"github.com/theauditor/vulnerable-api/internal/handlers"
	"github.com/theauditor/vulnerable-api/internal/models"
	"github.com/theauditor/vulnerable-api/internal/repository"
	"github.com/theauditor/vulnerable-api/internal/services"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

func main() {
	// Initialize GORM
	gormDB, err := gorm.Open(sqlite.Open("test.db"), &gorm.Config{})
	if err != nil {
		log.Fatal("Failed to connect to database:", err)
	}

	// Auto-migrate models
	gormDB.AutoMigrate(
		&models.User{},
		&models.Profile{},
		&models.Post{},
		&models.Comment{},
		&models.Tag{},
		&models.Session{},
		&models.AuditLog{},
		&models.FileUpload{},
	)

	// Get underlying sql.DB for raw queries
	sqlDB, err := gormDB.DB()
	if err != nil {
		log.Fatal("Failed to get sql.DB:", err)
	}

	// Initialize repository and handlers
	userRepo := repository.NewUserRepository(gormDB, sqlDB, nil)
	asyncService := services.NewAsyncService(sqlDB)

	// Start each framework on different ports
	go startGinServer(userRepo, sqlDB)
	go startEchoServer(userRepo, sqlDB)
	go startChiServer(userRepo, sqlDB)
	go startNetHTTPServer(userRepo, sqlDB)

	// Demo async service (goroutines, channels)
	demonstrateAsync(asyncService)

	// Keep main running
	select {}
}

// startGinServer starts the Gin HTTP server on port 8081
func startGinServer(userRepo *repository.UserRepository, db *sql.DB) {
	r := gin.Default()
	h := handlers.NewGinHandler(userRepo, db)

	// Vulnerable routes
	r.GET("/users", h.GetUser)
	r.GET("/users/:username", h.GetUserByUsername)
	r.POST("/users/search", h.SearchUsers)
	r.POST("/users", h.CreateUser)
	r.GET("/cmd", h.RunCommand)
	r.GET("/ping/:host", h.PingHost)
	r.GET("/profile", h.RenderProfile)
	r.GET("/script", h.RenderJS)
	r.GET("/download/:filename", h.DownloadFile)
	r.GET("/config", h.ReadConfig)
	r.POST("/save", h.SaveFile)
	r.POST("/complex", h.ComplexFlow)

	// Secure routes
	r.GET("/users/secure", h.GetUserSecure)
	r.GET("/cmd/secure", h.RunCommandSecure)

	fmt.Println("Gin server starting on :8081")
	r.Run(":8081")
}

// startEchoServer starts the Echo HTTP server on port 8082
func startEchoServer(userRepo *repository.UserRepository, db *sql.DB) {
	e := echo.New()
	h := handlers.NewEchoHandler(userRepo, db)

	// Vulnerable routes
	e.GET("/users", h.GetUser)
	e.GET("/users/:username", h.GetUserByUsername)
	e.POST("/users/search", h.SearchUsers)
	e.POST("/users", h.CreateUser)
	e.PUT("/users/:id", h.UpdateUser)
	e.GET("/diagnostic", h.RunDiagnostic)
	e.POST("/shell", h.ExecShell)
	e.GET("/files", h.ServeFile)
	e.DELETE("/files/:filename", h.DeleteFile)
	e.POST("/process", h.ProcessRequest)
	e.GET("/reports", h.ReportEndpoint)

	// Secure routes
	e.GET("/users/secure", h.GetUserSecure)

	fmt.Println("Echo server starting on :8082")
	e.Start(":8082")
}

// startChiServer starts the Chi HTTP server on port 8083
func startChiServer(userRepo *repository.UserRepository, db *sql.DB) {
	r := chi.NewRouter()
	r.Use(middleware.Logger)
	r.Use(middleware.Recoverer)

	h := handlers.NewChiHandler(userRepo, db)

	// Vulnerable routes
	r.Get("/users/{id}", h.GetUser)
	r.Get("/users/by-username/{username}", h.GetUserByUsername)
	r.Get("/users/search", h.SearchUsers)
	r.Post("/users", h.CreateUser)
	r.Put("/users/{id}", h.UpdateUser)
	r.Delete("/users/{id}", h.DeleteUser)
	r.Get("/cmd", h.RunCommand)
	r.Get("/sysinfo", h.SystemInfo)
	r.Get("/files/*", h.ServeFile)
	r.Post("/upload", h.UploadFile)
	r.Post("/audit/{action}", h.AuditLog)

	// Secure routes
	r.Get("/users/secure/{id}", h.GetUserSecure)
	r.Post("/users/secure", h.CreateUserSecure)

	fmt.Println("Chi server starting on :8083")
	http.ListenAndServe(":8083", r)
}

// startNetHTTPServer starts the standard net/http server on port 8084
func startNetHTTPServer(userRepo *repository.UserRepository, db *sql.DB) {
	h := handlers.NewNetHTTPHandler(userRepo, db)

	// Vulnerable routes
	http.HandleFunc("/users", h.GetUser)
	http.HandleFunc("/users/search", h.SearchUsers)
	http.HandleFunc("/users/create", h.CreateUser)
	http.HandleFunc("/users/update", h.UpdateUser)
	http.HandleFunc("/process", h.ProcessJSON)
	http.HandleFunc("/raw", h.RawBodyToSQL)
	http.HandleFunc("/cmd", h.RunCommand)
	http.HandleFunc("/shell", h.ShellExec)
	http.HandleFunc("/process-file", h.ProcessFile)
	http.HandleFunc("/files", h.ServeFile)
	http.HandleFunc("/files/", h.ReadFile)
	http.HandleFunc("/write", h.WriteFile)
	http.HandleFunc("/log", h.LogRequest)
	http.HandleFunc("/cookie", h.ProcessCookie)
	http.HandleFunc("/complex", h.ComplexHandler)

	// Secure routes
	http.HandleFunc("/users/secure", h.GetUserSecure)
	http.HandleFunc("/users/secure/create", h.CreateUserSecure)

	fmt.Println("net/http server starting on :8084")
	http.ListenAndServe(":8084", nil)
}

// demonstrateAsync shows goroutine/channel patterns
func demonstrateAsync(svc *services.AsyncService) {
	// Demonstrate goroutine patterns
	items := []string{"item1", "item2", "item3"}

	// Named function goroutine
	svc.ProcessItemsAsync(items)

	// Anonymous goroutine with captured variable (race condition)
	svc.ProcessWithAnonymous(items)

	// Fixed version with parameters
	svc.ProcessWithAnonymousFixed(items)

	// Fetch with WaitGroup
	urls := []string{"http://example1.com", "http://example2.com"}
	_ = svc.FetchAll(urls)

	// Worker pool with channels
	jobs := svc.WorkerPool(3)
	jobs <- "job1"
	jobs <- "job2"

	// Pipeline pattern
	input := []int{1, 2, 3, 4}
	pipeline := svc.Pipeline(input)
	for result := range pipeline {
		fmt.Println("Pipeline result:", result)
	}

	// Defer demonstrations
	svc.ResourceCleanup()
}
