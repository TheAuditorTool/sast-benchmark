// Package api provides HTTP handlers for the notification service
package api

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"strings"

	"github.com/gorilla/mux"
	"github.com/project-anarchy/go_notifications/internal/channels"
	"github.com/project-anarchy/go_notifications/internal/queue"
	"github.com/project-anarchy/go_notifications/internal/storage"
	"github.com/project-anarchy/go_notifications/internal/templates"
	"github.com/project-anarchy/go_notifications/pkg/utils"
)

// Handlers holds all HTTP handler dependencies
type Handlers struct {
	store      *storage.SQLiteStore
	dispatcher *channels.Dispatcher
	renderer   *templates.Renderer
	worker     *queue.Worker
}

// NewHandlers creates a new Handlers instance
func NewHandlers(store *storage.SQLiteStore, dispatcher *channels.Dispatcher, renderer *templates.Renderer, worker *queue.Worker) *Handlers {
	return &Handlers{
		store:      store,
		dispatcher: dispatcher,
		renderer:   renderer,
		worker:     worker,
	}
}

// NotificationRequest represents an incoming notification request
type NotificationRequest struct {
	Channel   string            `json:"channel"`   // email, webhook, slack, file
	Recipient string            `json:"recipient"` // email address, URL, or filename
	Subject   string            `json:"subject"`
	Message   string            `json:"message"`
	Template  string            `json:"template,omitempty"`
	Data      map[string]interface{} `json:"data,omitempty"`
	Priority  int               `json:"priority,omitempty"`
	Metadata  map[string]string `json:"metadata,omitempty"`
}

// BatchNotificationRequest represents a batch of notifications
type BatchNotificationRequest struct {
	Notifications []NotificationRequest `json:"notifications"`
	Async         bool                  `json:"async"`
}

// HealthCheck returns service health status
func (h *Handlers) HealthCheck(w http.ResponseWriter, r *http.Request) {
	status := map[string]interface{}{
		"status":      "healthy",
		"queue_size":  h.worker.QueueSize(),
		"db_status":   h.store.Ping(),
	}
	respondJSON(w, http.StatusOK, status)
}

// SendNotification handles immediate notification dispatch
// TAINT SOURCE: All request body fields
func (h *Handlers) SendNotification(w http.ResponseWriter, r *http.Request) {
	var req NotificationRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		respondError(w, http.StatusBadRequest, "Invalid request body: "+err.Error())
		return
	}

	// VULN: User input directly used without validation
	notification := &channels.Notification{
		Channel:   req.Channel,
		Recipient: req.Recipient, // TAINT: User-controlled recipient
		Subject:   req.Subject,   // TAINT: User-controlled subject
		Message:   req.Message,   // TAINT: User-controlled message body
		Metadata:  req.Metadata,
	}

	// If template specified, render it with user data
	if req.Template != "" {
		// TAINT FLOW: req.Template (user input) -> renderer.Render
		rendered, err := h.renderer.Render(req.Template, req.Data)
		if err != nil {
			respondError(w, http.StatusBadRequest, "Template error: "+err.Error())
			return
		}
		notification.Message = rendered
	}

	// Store notification in database
	id, err := h.store.SaveNotification(notification)
	if err != nil {
		respondError(w, http.StatusInternalServerError, "Failed to save notification")
		return
	}
	notification.ID = id

	// Dispatch immediately
	result, err := h.dispatcher.Dispatch(notification)
	if err != nil {
		h.store.UpdateStatus(id, "failed", err.Error())
		respondError(w, http.StatusInternalServerError, "Dispatch failed: "+err.Error())
		return
	}

	h.store.UpdateStatus(id, "sent", "")

	respondJSON(w, http.StatusOK, map[string]interface{}{
		"id":     id,
		"status": "sent",
		"result": result,
	})
}

// SendBatchNotification handles batch notification requests
func (h *Handlers) SendBatchNotification(w http.ResponseWriter, r *http.Request) {
	var req BatchNotificationRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		respondError(w, http.StatusBadRequest, "Invalid request body")
		return
	}

	results := make([]map[string]interface{}, 0, len(req.Notifications))

	for _, notif := range req.Notifications {
		notification := &channels.Notification{
			Channel:   notif.Channel,
			Recipient: notif.Recipient,
			Subject:   notif.Subject,
			Message:   notif.Message,
			Metadata:  notif.Metadata,
		}

		if req.Async {
			// Queue for background processing
			jobID, _ := h.worker.Enqueue(notification)
			results = append(results, map[string]interface{}{
				"queued": true,
				"job_id": jobID,
			})
		} else {
			// Process synchronously
			id, _ := h.store.SaveNotification(notification)
			result, err := h.dispatcher.Dispatch(notification)
			status := "sent"
			if err != nil {
				status = "failed"
			}
			results = append(results, map[string]interface{}{
				"id":     id,
				"status": status,
				"result": result,
			})
		}
	}

	respondJSON(w, http.StatusOK, map[string]interface{}{
		"processed": len(results),
		"results":   results,
	})
}

// SendTemplatedNotification renders a template and sends notification
// TAINT SOURCE: Template name and data from request
func (h *Handlers) SendTemplatedNotification(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Template  string                 `json:"template"`
		Channel   string                 `json:"channel"`
		Recipient string                 `json:"recipient"`
		Subject   string                 `json:"subject"`
		Data      map[string]interface{} `json:"data"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		respondError(w, http.StatusBadRequest, "Invalid request")
		return
	}

	// VULN: Template path traversal possible
	// req.Template could be "../../../etc/passwd"
	rendered, err := h.renderer.Render(req.Template, req.Data)
	if err != nil {
		respondError(w, http.StatusBadRequest, fmt.Sprintf("Template render failed: %v", err))
		return
	}

	notification := &channels.Notification{
		Channel:   req.Channel,
		Recipient: req.Recipient,
		Subject:   req.Subject,
		Message:   rendered, // TAINT SINK: Rendered template with user data
	}

	result, err := h.dispatcher.Dispatch(notification)
	if err != nil {
		respondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	respondJSON(w, http.StatusOK, result)
}

// ListNotifications returns stored notifications with filtering
// TAINT SOURCE: Query parameters
func (h *Handlers) ListNotifications(w http.ResponseWriter, r *http.Request) {
	// VULN: Query params used directly in SQL
	channel := r.URL.Query().Get("channel")
	status := r.URL.Query().Get("status")
	recipient := r.URL.Query().Get("recipient")
	limit := r.URL.Query().Get("limit")
	orderBy := r.URL.Query().Get("order_by") // VULN: SQL injection via ORDER BY

	// TAINT FLOW: Query params -> SQL query
	notifications, err := h.store.ListNotifications(channel, status, recipient, limit, orderBy)
	if err != nil {
		respondError(w, http.StatusInternalServerError, "Query failed: "+err.Error())
		return
	}

	respondJSON(w, http.StatusOK, notifications)
}

// TestWebhook sends a test webhook to a user-provided URL
// TAINT SOURCE: URL from request body - SSRF vulnerability
func (h *Handlers) TestWebhook(w http.ResponseWriter, r *http.Request) {
	var req struct {
		URL     string            `json:"url"`
		Method  string            `json:"method"`
		Headers map[string]string `json:"headers"`
		Body    string            `json:"body"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		respondError(w, http.StatusBadRequest, "Invalid request")
		return
	}

	// VULN: SSRF - No URL validation, internal network accessible
	// User can provide http://169.254.169.254/latest/meta-data/ (AWS)
	// Or http://localhost:6379/ (Redis) etc.
	result, err := h.dispatcher.WebhookChannel().SendToURL(
		req.URL,     // TAINT SINK: User-controlled URL
		req.Method,
		req.Headers, // TAINT: User-controlled headers
		req.Body,    // TAINT: User-controlled body
	)
	if err != nil {
		respondError(w, http.StatusBadGateway, "Webhook failed: "+err.Error())
		return
	}

	respondJSON(w, http.StatusOK, result)
}

// ExecuteHook runs a shell script hook
// TAINT SOURCE: Hook name and arguments - Command Injection
func (h *Handlers) ExecuteHook(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Hook      string   `json:"hook"`
		Arguments []string `json:"arguments"`
		Env       map[string]string `json:"env"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		respondError(w, http.StatusBadRequest, "Invalid request")
		return
	}

	// VULN: Path traversal in hook name
	hookPath := filepath.Join("./scripts/hooks", req.Hook)

	// VULN: No validation that hookPath is within allowed directory
	// req.Hook could be "../../malicious.sh" or "hook; rm -rf /"

	// Check if hook exists
	if _, err := os.Stat(hookPath); os.IsNotExist(err) {
		respondError(w, http.StatusNotFound, "Hook not found")
		return
	}

	// VULN: Command injection via arguments
	// Arguments are passed directly to shell
	cmd := exec.Command(hookPath, req.Arguments...)

	// VULN: User-controlled environment variables
	for key, value := range req.Env {
		cmd.Env = append(cmd.Env, fmt.Sprintf("%s=%s", key, value))
	}

	output, err := cmd.CombinedOutput()
	if err != nil {
		respondJSON(w, http.StatusOK, map[string]interface{}{
			"success": false,
			"output":  string(output),
			"error":   err.Error(),
		})
		return
	}

	respondJSON(w, http.StatusOK, map[string]interface{}{
		"success": true,
		"output":  string(output),
	})
}

// ReadLogFile reads a log file by name
// TAINT SOURCE: Filename from URL - Path Traversal
func (h *Handlers) ReadLogFile(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	filename := vars["filename"] // TAINT SOURCE

	// VULN: Path traversal - no sanitization
	// filename could be "../../../etc/passwd"
	logPath := filepath.Join("./logs", filename)

	// VULN: No validation that path is within logs directory
	content, err := os.ReadFile(logPath) // TAINT SINK
	if err != nil {
		respondError(w, http.StatusNotFound, "Log file not found")
		return
	}

	// Return raw content
	w.Header().Set("Content-Type", "text/plain")
	w.Write(content)
}

// ImportConfig imports configuration from uploaded file
// TAINT SOURCE: File upload content
func (h *Handlers) ImportConfig(w http.ResponseWriter, r *http.Request) {
	file, header, err := r.FormFile("config")
	if err != nil {
		respondError(w, http.StatusBadRequest, "No file uploaded")
		return
	}
	defer file.Close()

	// VULN: No file type validation
	content, _ := io.ReadAll(file)

	// VULN: Path from form used directly
	savePath := r.FormValue("save_path")
	if savePath == "" {
		savePath = filepath.Join("./config", header.Filename) // VULN: User-controlled filename
	}

	// VULN: Arbitrary file write
	if err := os.WriteFile(savePath, content, 0644); err != nil {
		respondError(w, http.StatusInternalServerError, "Failed to save config")
		return
	}

	respondJSON(w, http.StatusOK, map[string]string{
		"saved": savePath,
	})
}

// SearchNotifications performs full-text search
// TAINT SOURCE: Search query
func (h *Handlers) SearchNotifications(w http.ResponseWriter, r *http.Request) {
	query := r.URL.Query().Get("q")

	// VULN: Search query used in SQL LIKE without escaping
	results, err := h.store.Search(query)
	if err != nil {
		respondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	respondJSON(w, http.StatusOK, results)
}

// ExportNotifications exports notifications to a file
// TAINT FLOW: User specifies output format and path
func (h *Handlers) ExportNotifications(w http.ResponseWriter, r *http.Request) {
	format := r.URL.Query().Get("format") // csv, json, xml
	output := r.URL.Query().Get("output") // VULN: Arbitrary path

	notifications, _ := h.store.ListNotifications("", "", "", "1000", "id")

	var content []byte
	switch format {
	case "csv":
		content = utils.ToCSV(notifications)
	case "xml":
		content = utils.ToXML(notifications)
	default:
		content, _ = json.Marshal(notifications)
	}

	if output != "" {
		// VULN: Arbitrary file write to user-specified path
		os.WriteFile(output, content, 0644)
		respondJSON(w, http.StatusOK, map[string]string{"exported": output})
	} else {
		w.Header().Set("Content-Type", "application/octet-stream")
		w.Write(content)
	}
}

// ProcessCallback handles webhook callbacks
// TAINT SOURCE: Entire request body and headers
func (h *Handlers) ProcessCallback(w http.ResponseWriter, r *http.Request) {
	callbackID := mux.Vars(r)["id"]

	// Read callback body
	body, _ := io.ReadAll(r.Body)

	// VULN: Callback data processed without validation
	var callbackData map[string]interface{}
	json.Unmarshal(body, &callbackData)

	// Log callback with user-controlled data
	logEntry := fmt.Sprintf("[CALLBACK %s] Headers: %v Body: %s\n",
		callbackID,
		r.Header, // TAINT: User-controlled headers
		string(body), // TAINT: User-controlled body
	)

	// VULN: Log injection - newlines in body can inject fake log entries
	logPath := filepath.Join("./logs", "callbacks.log")
	f, _ := os.OpenFile(logPath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	f.WriteString(logEntry)
	f.Close()

	// Execute callback hook if configured
	if hookCmd := r.Header.Get("X-Callback-Hook"); hookCmd != "" {
		// VULN: Header value used in command execution
		go func() {
			cmd := exec.Command("sh", "-c", hookCmd) // TAINT SINK: Command injection
			cmd.Run()
		}()
	}

	respondJSON(w, http.StatusOK, map[string]string{"status": "processed"})
}

// Utility functions
func respondJSON(w http.ResponseWriter, status int, data interface{}) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	json.NewEncoder(w).Encode(data)
}

func respondError(w http.ResponseWriter, status int, message string) {
	respondJSON(w, status, map[string]string{"error": message})
}

// DebugRequest logs full request details - useful for debugging
// VULN: Logs sensitive information
func (h *Handlers) DebugRequest(w http.ResponseWriter, r *http.Request) {
	body, _ := io.ReadAll(r.Body)

	debug := map[string]interface{}{
		"method":      r.Method,
		"url":         r.URL.String(),
		"headers":     r.Header,
		"body":        string(body),
		"remote_addr": r.RemoteAddr,
		"cookies":     r.Cookies(),
	}

	// VULN: Dumps all headers including Authorization
	respondJSON(w, http.StatusOK, debug)
}

// ProxyRequest proxies a request to another service
// VULN: Open redirect / SSRF
func (h *Handlers) ProxyRequest(w http.ResponseWriter, r *http.Request) {
	targetURL := r.URL.Query().Get("target")

	// VULN: No URL validation - SSRF
	if !strings.HasPrefix(targetURL, "http") {
		targetURL = "http://" + targetURL
	}

	resp, err := http.Get(targetURL) // TAINT SINK: User-controlled URL
	if err != nil {
		respondError(w, http.StatusBadGateway, err.Error())
		return
	}
	defer resp.Body.Close()

	// Copy response
	body, _ := io.ReadAll(resp.Body)
	w.WriteHeader(resp.StatusCode)
	w.Write(body)
}

// LookupUser fetches user details from the admin service for notification enrichment
// CROSS-SERVICE CALL: go_notifications -> beego_admin
func (h *Handlers) LookupUser(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	if username == "" {
		respondError(w, http.StatusBadRequest, "username required")
		return
	}

	// Fetch user from admin service - CROSS-SERVICE HTTP CALL
	userData, err := h.fetchUserFromAdminService(username)
	if err != nil {
		respondError(w, http.StatusBadGateway, "Failed to fetch user: "+err.Error())
		return
	}

	respondJSON(w, http.StatusOK, userData)
}

// fetchUserFromAdminService makes a cross-service call to beego_admin
// This enables taint tracking across service boundaries
func (h *Handlers) fetchUserFromAdminService(username string) (map[string]interface{}, error) {
	// CROSS-SERVICE: Call beego_admin user search endpoint
	// Uses literal URL for static analysis to detect cross-service data flow
	resp, err := http.Get("http://localhost:8081/users/search?name=" + username)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	var userData map[string]interface{}
	if err := json.NewDecoder(resp.Body).Decode(&userData); err != nil {
		return nil, err
	}

	return userData, nil
}

// EnrichAndSendNotification fetches user preferences and sends notification
// CROSS-SERVICE FLOW: Request -> Admin Service -> Notification Dispatch
func (h *Handlers) EnrichAndSendNotification(w http.ResponseWriter, r *http.Request) {
	var req NotificationRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		respondError(w, http.StatusBadRequest, "Invalid request body")
		return
	}

	// CROSS-SERVICE: Fetch user preferences from admin service
	userPrefs, err := h.fetchUserPreferences(req.Recipient)
	if err != nil {
		// Fall through - send with defaults
		userPrefs = map[string]interface{}{"channel": "email"}
	}

	// Use preferred channel if not specified
	if req.Channel == "" {
		if prefChannel, ok := userPrefs["preferred_channel"].(string); ok {
			req.Channel = prefChannel
		}
	}

	notification := &channels.Notification{
		Channel:   req.Channel,
		Recipient: req.Recipient,
		Subject:   req.Subject,
		Message:   req.Message,
		Metadata:  req.Metadata,
	}

	id, _ := h.store.SaveNotification(notification)
	notification.ID = id

	result, err := h.dispatcher.Dispatch(notification)
	if err != nil {
		respondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	respondJSON(w, http.StatusOK, map[string]interface{}{
		"id":         id,
		"status":     "sent",
		"result":     result,
		"user_prefs": userPrefs,
	})
}

// fetchUserPreferences gets user notification preferences from admin service
func (h *Handlers) fetchUserPreferences(userID string) (map[string]interface{}, error) {
	// CROSS-SERVICE: Call beego_admin to get user preferences
	// http.NewRequest pattern for POST/PUT operations
	req, err := http.NewRequest("GET", "http://localhost:8081/users/by-name?username="+userID, nil)
	if err != nil {
		return nil, err
	}
	req.Header.Set("X-Service-Name", "go_notifications")
	req.Header.Set("Accept", "application/json")

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	var prefs map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&prefs)
	return prefs, nil
}
