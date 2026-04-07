// Package main - Entry point for the Go Notification Service
// This service handles notifications via multiple channels: email, webhook, slack, file
// Part of the Project Anarchy test suite for TheAuditor
package main

import (
	"context"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/project-anarchy/go_notifications/internal/api"
	"github.com/project-anarchy/go_notifications/internal/channels"
	"github.com/project-anarchy/go_notifications/internal/config"
	"github.com/project-anarchy/go_notifications/internal/queue"
	"github.com/project-anarchy/go_notifications/internal/storage"
	"github.com/project-anarchy/go_notifications/internal/templates"
)

func main() {
	// Load configuration
	cfg, err := config.Load("config.yaml")
	if err != nil {
		log.Printf("Warning: Could not load config file, using defaults: %v", err)
		cfg = config.Default()
	}

	// Initialize storage layer
	store, err := storage.NewSQLiteStore(cfg.DatabasePath)
	if err != nil {
		log.Fatalf("Failed to initialize storage: %v", err)
	}
	defer store.Close()

	// Initialize template renderer
	renderer := templates.NewRenderer(cfg.TemplatesDir)

	// Initialize notification channels
	emailChannel := channels.NewEmailChannel(cfg.SMTP)
	webhookChannel := channels.NewWebhookChannel(cfg.WebhookTimeout)
	slackChannel := channels.NewSlackChannel(cfg.SlackWebhook)
	fileChannel := channels.NewFileChannel(cfg.LogDir)

	// Create channel dispatcher
	dispatcher := channels.NewDispatcher(
		emailChannel,
		webhookChannel,
		slackChannel,
		fileChannel,
	)

	// Initialize background queue worker
	worker := queue.NewWorker(store, dispatcher, renderer, cfg.WorkerCount)
	workerCtx, workerCancel := context.WithCancel(context.Background())
	go worker.Start(workerCtx)

	// Initialize API handlers
	handlers := api.NewHandlers(store, dispatcher, renderer, worker)

	// Setup router
	router := api.NewRouter(handlers, cfg.APIKey)

	// Create HTTP server
	server := &http.Server{
		Addr:         cfg.ListenAddr,
		Handler:      router,
		ReadTimeout:  15 * time.Second,
		WriteTimeout: 15 * time.Second,
		IdleTimeout:  60 * time.Second,
	}

	// Start server in goroutine
	go func() {
		log.Printf("Starting Go Notification Service on %s", cfg.ListenAddr)
		log.Printf("Endpoints:")
		log.Printf("  POST /api/notify          - Send immediate notification")
		log.Printf("  POST /api/notify/batch    - Queue batch notifications")
		log.Printf("  POST /api/notify/template - Send templated notification")
		log.Printf("  GET  /api/notifications   - List notifications")
		log.Printf("  GET  /api/health          - Health check")
		log.Printf("  POST /api/webhook/test    - Test webhook delivery")
		log.Printf("  POST /api/hooks/execute   - Execute shell hook")
		log.Printf("  GET  /api/logs/:filename  - Read log file")

		if err := server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("Server failed: %v", err)
		}
	}()

	// Wait for interrupt signal
	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit

	log.Println("Shutting down server...")

	// Cancel worker context
	workerCancel()

	// Graceful shutdown with timeout
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	if err := server.Shutdown(ctx); err != nil {
		log.Fatalf("Server forced to shutdown: %v", err)
	}

	log.Println("Server exited properly")
}
