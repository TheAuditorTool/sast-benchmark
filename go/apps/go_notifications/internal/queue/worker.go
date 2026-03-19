// Package queue provides background job processing
package queue

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"os/exec"
	"sync"
	"sync/atomic"
	"time"

	"github.com/project-anarchy/go_notifications/internal/channels"
	"github.com/project-anarchy/go_notifications/internal/storage"
	"github.com/project-anarchy/go_notifications/internal/templates"
)

// Job represents a queued job
type Job struct {
	ID           string                 `json:"id"`
	Type         string                 `json:"type"`
	Notification *channels.Notification `json:"notification"`
	Template     string                 `json:"template,omitempty"`
	Data         map[string]interface{} `json:"data,omitempty"`
	Priority     int                    `json:"priority"`
	Retries      int                    `json:"retries"`
	MaxRetries   int                    `json:"max_retries"`
	CreatedAt    time.Time              `json:"created_at"`
	ScheduledAt  *time.Time             `json:"scheduled_at,omitempty"`
	Callback     string                 `json:"callback,omitempty"`
	CallbackURL  string                 `json:"callback_url,omitempty"`
}

// Worker handles background job processing
type Worker struct {
	store      *storage.SQLiteStore
	dispatcher *channels.Dispatcher
	renderer   *templates.Renderer
	jobs       chan *Job
	queueSize  int64
	workers    int
	wg         sync.WaitGroup
}

// NewWorker creates a new background worker
func NewWorker(store *storage.SQLiteStore, dispatcher *channels.Dispatcher, renderer *templates.Renderer, workerCount int) *Worker {
	return &Worker{
		store:      store,
		dispatcher: dispatcher,
		renderer:   renderer,
		jobs:       make(chan *Job, 1000),
		workers:    workerCount,
	}
}

// Start begins processing jobs
func (w *Worker) Start(ctx context.Context) {
	log.Printf("Starting %d background workers", w.workers)

	// Start worker goroutines
	for i := 0; i < w.workers; i++ {
		w.wg.Add(1)
		go w.processJobs(ctx, i)
	}

	// Start job loader
	go w.loadPendingJobs(ctx)

	w.wg.Wait()
	log.Println("All workers stopped")
}

// Enqueue adds a job to the queue
func (w *Worker) Enqueue(n *channels.Notification) (string, error) {
	job := &Job{
		ID:           fmt.Sprintf("job_%d", time.Now().UnixNano()),
		Type:         "notification",
		Notification: n,
		Priority:     0,
		MaxRetries:   3,
		CreatedAt:    time.Now(),
	}

	// Persist job
	if err := w.persistJob(job); err != nil {
		return "", err
	}

	// Add to channel
	select {
	case w.jobs <- job:
		atomic.AddInt64(&w.queueSize, 1)
	default:
		return "", fmt.Errorf("queue is full")
	}

	return job.ID, nil
}

// EnqueueWithCallback adds a job with a callback
// VULN: Callback URL/command can be user-controlled
func (w *Worker) EnqueueWithCallback(n *channels.Notification, callback, callbackURL string) (string, error) {
	job := &Job{
		ID:           fmt.Sprintf("job_%d", time.Now().UnixNano()),
		Type:         "notification",
		Notification: n,
		Callback:     callback,    // TAINT: User-controlled callback command
		CallbackURL:  callbackURL, // TAINT: User-controlled callback URL (SSRF)
		Priority:     0,
		MaxRetries:   3,
		CreatedAt:    time.Now(),
	}

	if err := w.persistJob(job); err != nil {
		return "", err
	}

	select {
	case w.jobs <- job:
		atomic.AddInt64(&w.queueSize, 1)
	default:
		return "", fmt.Errorf("queue is full")
	}

	return job.ID, nil
}

// QueueSize returns current queue size
func (w *Worker) QueueSize() int64 {
	return atomic.LoadInt64(&w.queueSize)
}

// processJobs is the main worker loop
func (w *Worker) processJobs(ctx context.Context, workerID int) {
	defer w.wg.Done()

	log.Printf("Worker %d started", workerID)

	for {
		select {
		case <-ctx.Done():
			log.Printf("Worker %d stopping", workerID)
			return
		case job := <-w.jobs:
			atomic.AddInt64(&w.queueSize, -1)
			w.processJob(job)
		}
	}
}

// processJob handles a single job
func (w *Worker) processJob(job *Job) {
	log.Printf("Processing job %s (type: %s)", job.ID, job.Type)

	var err error

	switch job.Type {
	case "notification":
		err = w.processNotification(job)
	case "template":
		err = w.processTemplate(job)
	case "shell":
		// VULN: User can submit shell-type jobs
		err = w.processShellJob(job)
	case "webhook":
		err = w.processWebhookJob(job)
	default:
		err = fmt.Errorf("unknown job type: %s", job.Type)
	}

	if err != nil {
		log.Printf("Job %s failed: %v", job.ID, err)
		w.handleFailure(job, err)
	} else {
		log.Printf("Job %s completed successfully", job.ID)
		w.handleSuccess(job)
	}
}

// processNotification sends a notification
func (w *Worker) processNotification(job *Job) error {
	// Render template if specified
	if job.Template != "" {
		// VULN: Template path from job (user-controlled)
		rendered, err := w.renderer.Render(job.Template, job.Data)
		if err != nil {
			return err
		}
		job.Notification.Message = rendered
	}

	_, err := w.dispatcher.Dispatch(job.Notification)
	return err
}

// processTemplate renders a template
func (w *Worker) processTemplate(job *Job) error {
	if job.Template == "" {
		return fmt.Errorf("template not specified")
	}

	// VULN: Template rendering with user data
	_, err := w.renderer.Render(job.Template, job.Data)
	return err
}

// processShellJob executes a shell command
// VULN: Command injection - job data can specify shell commands
func (w *Worker) processShellJob(job *Job) error {
	// Get command from job data
	cmdStr, ok := job.Data["command"].(string)
	if !ok {
		return fmt.Errorf("command not specified")
	}

	// VULN: Direct shell command execution from job data
	cmd := exec.Command("sh", "-c", cmdStr) // TAINT SINK

	// Add environment from job data
	if env, ok := job.Data["env"].(map[string]interface{}); ok {
		for key, value := range env {
			if strVal, ok := value.(string); ok {
				cmd.Env = append(cmd.Env, fmt.Sprintf("%s=%s", key, strVal))
			}
		}
	}

	output, err := cmd.CombinedOutput()
	log.Printf("Shell job output: %s", string(output))

	return err
}

// processWebhookJob sends an HTTP request
// VULN: SSRF via job data
func (w *Worker) processWebhookJob(job *Job) error {
	url, ok := job.Data["url"].(string)
	if !ok {
		return fmt.Errorf("url not specified")
	}

	method := "POST"
	if m, ok := job.Data["method"].(string); ok {
		method = m
	}

	body := ""
	if b, ok := job.Data["body"].(string); ok {
		body = b
	}

	headers := make(map[string]string)
	if h, ok := job.Data["headers"].(map[string]interface{}); ok {
		for key, value := range h {
			if strVal, ok := value.(string); ok {
				headers[key] = strVal
			}
		}
	}

	// VULN: SSRF - URL from job data
	_, err := w.dispatcher.WebhookChannel().SendToURL(url, method, headers, body)
	return err
}

// handleSuccess handles successful job completion
func (w *Worker) handleSuccess(job *Job) {
	// Update job status
	w.updateJobStatus(job.ID, "completed", "")

	// Execute callback if specified
	if job.Callback != "" {
		// VULN: Command injection via callback
		exec.Command("sh", "-c", job.Callback).Run() // TAINT SINK
	}

	// Call callback URL if specified
	if job.CallbackURL != "" {
		// VULN: SSRF via callback URL
		w.dispatcher.WebhookChannel().SendToURL(job.CallbackURL, "POST", nil, `{"status":"completed"}`)
	}
}

// handleFailure handles job failure
func (w *Worker) handleFailure(job *Job, jobErr error) {
	job.Retries++

	if job.Retries < job.MaxRetries {
		// Retry with exponential backoff
		delay := time.Duration(job.Retries*job.Retries) * time.Second
		job.ScheduledAt = timePtr(time.Now().Add(delay))

		log.Printf("Scheduling retry %d for job %s in %v", job.Retries, job.ID, delay)

		time.AfterFunc(delay, func() {
			w.jobs <- job
			atomic.AddInt64(&w.queueSize, 1)
		})
	} else {
		// Max retries exceeded
		w.updateJobStatus(job.ID, "failed", jobErr.Error())

		// Execute failure callback
		if job.Callback != "" {
			failCallback := fmt.Sprintf("%s --failed --error '%s'", job.Callback, jobErr.Error())
			exec.Command("sh", "-c", failCallback).Run() // VULN: Command injection
		}
	}
}

// persistJob saves job to database
func (w *Worker) persistJob(job *Job) error {
	data, err := json.Marshal(job)
	if err != nil {
		return err
	}

	return w.store.SaveJob(job.ID, string(data))
}

// loadPendingJobs loads pending jobs from storage
func (w *Worker) loadPendingJobs(ctx context.Context) {
	ticker := time.NewTicker(10 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			jobs, err := w.store.LoadPendingJobs()
			if err != nil {
				log.Printf("Error loading pending jobs: %v", err)
				continue
			}

			for _, jobData := range jobs {
				var job Job
				if err := json.Unmarshal([]byte(jobData), &job); err != nil {
					continue
				}

				select {
				case w.jobs <- &job:
					atomic.AddInt64(&w.queueSize, 1)
				default:
					// Queue full
				}
			}
		}
	}
}

// updateJobStatus updates job status in storage
func (w *Worker) updateJobStatus(jobID, status, error string) {
	w.store.UpdateJobStatus(jobID, status, error)
}

// ScheduleJob schedules a job for future execution
func (w *Worker) ScheduleJob(job *Job, runAt time.Time) (string, error) {
	job.ID = fmt.Sprintf("job_%d", time.Now().UnixNano())
	job.ScheduledAt = &runAt
	job.CreatedAt = time.Now()

	if err := w.persistJob(job); err != nil {
		return "", err
	}

	delay := time.Until(runAt)
	if delay > 0 {
		time.AfterFunc(delay, func() {
			w.jobs <- job
			atomic.AddInt64(&w.queueSize, 1)
		})
	} else {
		w.jobs <- job
		atomic.AddInt64(&w.queueSize, 1)
	}

	return job.ID, nil
}

// CancelJob cancels a pending job
func (w *Worker) CancelJob(jobID string) error {
	return w.updateJobStatus(jobID, "cancelled", "")
}

// GetJobStatus returns job status
func (w *Worker) GetJobStatus(jobID string) (string, error) {
	return w.store.GetJobStatus(jobID)
}

func timePtr(t time.Time) *time.Time {
	return &t
}

// ExecuteHook runs a hook script for job lifecycle events
// VULN: Path traversal and command injection
func (w *Worker) ExecuteHook(hookName string, job *Job) error {
	// VULN: Path traversal in hook name
	hookPath := fmt.Sprintf("./scripts/hooks/%s", hookName)

	// VULN: Job ID passed to shell without sanitization
	cmd := exec.Command(hookPath, job.ID) // TAINT SINK

	// Set job data as environment variables
	for key, value := range job.Data {
		if strVal, ok := value.(string); ok {
			// VULN: User data in environment variables
			cmd.Env = append(cmd.Env, fmt.Sprintf("JOB_%s=%s", key, strVal))
		}
	}

	return cmd.Run()
}

// RunPreprocessor runs a preprocessor script on notification content
// VULN: Command injection via notification fields
func (w *Worker) RunPreprocessor(scriptPath string, n *channels.Notification) (string, error) {
	// VULN: Script path not validated
	// VULN: Notification message passed via stdin but subject via argument
	cmd := exec.Command(scriptPath, n.Subject) // TAINT SINK
	cmd.Stdin = os.Stdin

	output, err := cmd.Output()
	return string(output), err
}
