package services

import (
	"context"
	"database/sql"
	"fmt"
	"sync"
	"time"
	"unsafe"
)

// AsyncService handles asynchronous operations with goroutines and channels
// This file demonstrates concurrency patterns for TheAuditor's Go analysis
type AsyncService struct {
	db    *sql.DB
	cache map[string]interface{}
	mu    sync.Mutex
}

// NewAsyncService creates a new async service
func NewAsyncService(db *sql.DB) *AsyncService {
	return &AsyncService{
		db:    db,
		cache: make(map[string]interface{}),
	}
}

// ===============================================
// GOROUTINE PATTERNS - For go_goroutines table
// ===============================================

// ProcessItemsAsync - Spawns goroutines for parallel processing
func (s *AsyncService) ProcessItemsAsync(items []string) {
	for i, item := range items {
		// GOROUTINE: Named function reference
		go s.processItem(item, i)
	}
}

// processItem is called by goroutine
func (s *AsyncService) processItem(item string, index int) {
	time.Sleep(100 * time.Millisecond)
	fmt.Printf("Processed item %d: %s\n", index, item)
}

// ProcessWithAnonymous - VULNERABLE: Captured loop variable
// This is the classic Go race condition pattern
func (s *AsyncService) ProcessWithAnonymous(items []string) {
	for i, v := range items {
		// GOROUTINE: Anonymous function with captured loop variables
		// RACE CONDITION: 'v' and 'i' are captured by reference
		go func() {
			// v and i will have unexpected values (usually the last iteration)
			fmt.Printf("Item %d: %s\n", i, v)
		}()
	}
}

// ProcessWithAnonymousFixed - SECURE: Loop variable passed as parameter
func (s *AsyncService) ProcessWithAnonymousFixed(items []string) {
	for i, v := range items {
		// GOROUTINE: Anonymous function with parameters
		// SAFE: v and i are passed as parameters (copied)
		go func(index int, value string) {
			fmt.Printf("Item %d: %s\n", index, value)
		}(i, v)
	}
}

// FetchAll - Multiple goroutines with wait group
func (s *AsyncService) FetchAll(urls []string) []string {
	var wg sync.WaitGroup
	results := make([]string, len(urls))

	for i, url := range urls {
		wg.Add(1)
		// GOROUTINE: With WaitGroup
		go func(idx int, u string) {
			defer wg.Done()
			results[idx] = s.fetchURL(u)
		}(i, url)
	}

	wg.Wait()
	return results
}

func (s *AsyncService) fetchURL(url string) string {
	time.Sleep(50 * time.Millisecond)
	return fmt.Sprintf("Response from %s", url)
}

// ===============================================
// CHANNEL PATTERNS - For go_channels table
// ===============================================

// WorkerPool - Creates channels and worker goroutines
func (s *AsyncService) WorkerPool(numWorkers int) chan<- string {
	// CHANNEL: Buffered job channel
	jobs := make(chan string, 100)

	// CHANNEL: Unbuffered result channel
	results := make(chan string)

	// Spawn worker goroutines
	for w := 1; w <= numWorkers; w++ {
		// GOROUTINE: Worker function
		go s.worker(w, jobs, results)
	}

	return jobs
}

// worker processes jobs from channel
func (s *AsyncService) worker(id int, jobs <-chan string, results chan<- string) {
	for job := range jobs {
		// CHANNEL OPERATION: Receive from jobs
		processed := fmt.Sprintf("Worker %d processed: %s", id, job)

		// CHANNEL OPERATION: Send to results
		results <- processed
	}
}

// Pipeline - Demonstrates channel pipeline pattern
func (s *AsyncService) Pipeline(input []int) <-chan int {
	// Stage 1: Generator
	gen := func(nums []int) <-chan int {
		out := make(chan int)
		go func() {
			for _, n := range nums {
				// CHANNEL OPERATION: Send
				out <- n
			}
			close(out)
		}()
		return out
	}

	// Stage 2: Square
	sq := func(in <-chan int) <-chan int {
		out := make(chan int)
		go func() {
			for n := range in {
				// CHANNEL OPERATION: Receive and send
				out <- n * n
			}
			close(out)
		}()
		return out
	}

	// Stage 3: Double
	double := func(in <-chan int) <-chan int {
		out := make(chan int)
		go func() {
			for n := range in {
				// CHANNEL OPERATION: Receive and send
				out <- n * 2
			}
			close(out)
		}()
		return out
	}

	// Connect pipeline
	return double(sq(gen(input)))
}

// FanOut - Demonstrates fan-out pattern
func (s *AsyncService) FanOut(input <-chan string, numWorkers int) []<-chan string {
	outputs := make([]<-chan string, numWorkers)

	for i := 0; i < numWorkers; i++ {
		out := make(chan string)
		outputs[i] = out

		// GOROUTINE: Fan-out worker
		go func(ch chan<- string) {
			for item := range input {
				// Process and send to output
				ch <- fmt.Sprintf("Processed: %s", item)
			}
			close(ch)
		}(out)
	}

	return outputs
}

// ===============================================
// DEFER PATTERNS - For go_defer_statements table
// ===============================================

// DatabaseOperation - Demonstrates defer for cleanup
func (s *AsyncService) DatabaseOperation(userID string) error {
	tx, err := s.db.Begin()
	if err != nil {
		return err
	}

	// DEFER: Rollback on error
	defer func() {
		if err != nil {
			tx.Rollback()
		}
	}()

	// Perform operations
	_, err = tx.Exec("UPDATE users SET last_seen = NOW() WHERE id = ?", userID)
	if err != nil {
		return err
	}

	// DEFER: Commit at end
	defer tx.Commit()

	return nil
}

// FileOperation - Demonstrates defer for file cleanup
func (s *AsyncService) FileOperation(filename string) error {
	// DEFER: Close file
	// Note: This would need actual file operations
	defer func() {
		fmt.Printf("Closing file: %s\n", filename)
	}()

	// Process file
	fmt.Printf("Processing file: %s\n", filename)
	return nil
}

// ResourceCleanup - Multiple defers (LIFO order)
func (s *AsyncService) ResourceCleanup() {
	// DEFER #1: Will execute last
	defer fmt.Println("Cleanup 1: Database connection")

	// DEFER #2: Will execute second
	defer fmt.Println("Cleanup 2: Cache flush")

	// DEFER #3: Will execute first
	defer fmt.Println("Cleanup 3: Temp files")

	fmt.Println("Main operation")
}

// ===============================================
// CONTEXT PATTERNS - For cancellation/timeout
// ===============================================

// LongRunningTask - Uses context for cancellation
func (s *AsyncService) LongRunningTask(ctx context.Context, data string) error {
	// GOROUTINE: With context cancellation
	resultCh := make(chan string)
	errCh := make(chan error)

	go func() {
		// Simulate long task
		time.Sleep(5 * time.Second)
		resultCh <- fmt.Sprintf("Processed: %s", data)
	}()

	select {
	case result := <-resultCh:
		fmt.Println(result)
		return nil
	case err := <-errCh:
		return err
	case <-ctx.Done():
		// Context cancelled or timed out
		return ctx.Err()
	}
}

// TimeoutTask - Uses context with timeout
func (s *AsyncService) TimeoutTask(data string, timeout time.Duration) error {
	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()

	return s.LongRunningTask(ctx, data)
}

// ===============================================
// RACE CONDITION EXAMPLES - For security analysis
// ===============================================

// UnsafeCounter - VULNERABLE: Race condition on shared counter
type UnsafeCounter struct {
	value int
}

// Increment - VULNERABLE: Not thread-safe
func (c *UnsafeCounter) Increment() {
	// RACE CONDITION: Read-modify-write without synchronization
	c.value++
}

// Get - VULNERABLE: Not thread-safe read
func (c *UnsafeCounter) Get() int {
	return c.value
}

// SafeCounter - SECURE: Uses mutex for thread safety
type SafeCounter struct {
	mu    sync.Mutex
	value int
}

// Increment - SECURE: Protected by mutex
func (c *SafeCounter) Increment() {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.value++
}

// Get - SECURE: Protected by mutex
func (c *SafeCounter) Get() int {
	c.mu.Lock()
	defer c.mu.Unlock()
	return c.value
}

// UnsafeCacheUpdate - VULNERABLE: Race on map access
func (s *AsyncService) UnsafeCacheUpdate(key string, value interface{}) {
	// RACE CONDITION: Map access without synchronization
	// Multiple goroutines can read/write simultaneously
	s.cache[key] = value
}

// SafeCacheUpdate - SECURE: Protected by mutex
func (s *AsyncService) SafeCacheUpdate(key string, value interface{}) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.cache[key] = value
}

// ===============================================
// ERROR RETURN PATTERNS - For go_error_returns
// ===============================================

// FetchUser - Returns (User, error)
func (s *AsyncService) FetchUser(id string) (*User, error) {
	if id == "" {
		return nil, fmt.Errorf("user id cannot be empty")
	}

	// Simulate database lookup
	return &User{ID: id, Name: "Test User"}, nil
}

// User struct for examples
type User struct {
	ID   string
	Name string
}

// SaveUser - Returns error only
func (s *AsyncService) SaveUser(user *User) error {
	if user == nil {
		return fmt.Errorf("user cannot be nil")
	}
	// Simulate save
	return nil
}

// ValidateAndSave - Demonstrates error propagation
func (s *AsyncService) ValidateAndSave(id, name string) error {
	if id == "" {
		return fmt.Errorf("validation failed: id required")
	}

	user := &User{ID: id, Name: name}

	// Error propagation
	if err := s.SaveUser(user); err != nil {
		return fmt.Errorf("save failed: %w", err)
	}

	return nil
}

// BatchProcess - Returns (int, []error) for partial success
func (s *AsyncService) BatchProcess(items []string) (int, []error) {
	var errors []error
	successCount := 0

	for _, item := range items {
		if err := s.processItemWithError(item); err != nil {
			errors = append(errors, err)
		} else {
			successCount++
		}
	}

	return successCount, errors
}

func (s *AsyncService) processItemWithError(item string) error {
	if item == "" {
		return fmt.Errorf("empty item")
	}
	return nil
}

// ===============================================
// PANIC/RECOVER PATTERNS - For go_panic_calls, go_recover_calls
// ===============================================

// MustParseConfig panics if config is invalid - used for startup validation
func MustParseConfig(configData []byte) *ServiceConfig {
	if len(configData) == 0 {
		panic("config data cannot be empty")
	}

	config := &ServiceConfig{}
	// In real code, would unmarshal JSON here
	if config.MaxWorkers <= 0 {
		panic(fmt.Sprintf("invalid max_workers: %d", config.MaxWorkers))
	}

	return config
}

// ServiceConfig holds service configuration
type ServiceConfig struct {
	MaxWorkers int
	Timeout    time.Duration
}

// SafeExecute wraps a function with panic recovery
func (s *AsyncService) SafeExecute(fn func() error) (err error) {
	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("panic recovered: %v", r)
		}
	}()

	return fn()
}

// AssertNotNil panics if value is nil - for invariant checking
func AssertNotNil(value interface{}, name string) {
	if value == nil {
		panic(fmt.Sprintf("assertion failed: %s must not be nil", name))
	}
}

// ===============================================
// UNSAFE PATTERNS - For go_unsafe_usage
// ===============================================

// StringToBytes converts string to []byte without allocation using unsafe
// This is a common optimization pattern in high-performance Go code
func StringToBytes(s string) []byte {
	return unsafe.Slice(unsafe.StringData(s), len(s))
}

// BytesToString converts []byte to string without allocation using unsafe
func BytesToString(b []byte) string {
	if len(b) == 0 {
		return ""
	}
	return unsafe.String(&b[0], len(b))
}

// GetStructSize returns the memory size of a struct using unsafe.Sizeof
func GetStructSize(v interface{}) uintptr {
	return unsafe.Sizeof(v)
}

// PointerArithmetic demonstrates unsafe pointer operations for memory mapping
func PointerArithmetic(data []byte, offset int) *byte {
	if offset >= len(data) {
		return nil
	}
	ptr := unsafe.Pointer(&data[0])
	return (*byte)(unsafe.Add(ptr, offset))
}

// ===============================================
// ERROR IGNORED PATTERNS - For go_error_ignored
// ===============================================

// CleanupResources demonstrates intentionally ignored errors during cleanup
func (s *AsyncService) CleanupResources() {
	// During cleanup, we log but don't propagate errors
	// This is a common pattern for graceful shutdown

	// Ignored error: flush cache (best effort)
	_ = s.flushCache()

	// Ignored error: close connections (best effort)
	_, _ = s.closeConnections()

	// Ignored error: remove temp files
	_ = s.removeTempFiles()
}

func (s *AsyncService) flushCache() error {
	// Simulate cache flush
	return nil
}

func (s *AsyncService) closeConnections() (int, error) {
	// Simulate closing connections, returns count closed
	return 0, nil
}

func (s *AsyncService) removeTempFiles() error {
	// Simulate temp file removal
	return nil
}

// FireAndForget launches a goroutine for non-critical background work
func (s *AsyncService) FireAndForget(task func() error) {
	go func() {
		// Error intentionally ignored - this is fire-and-forget
		_ = task()
	}()
}

// LogWithFallback logs a message, ignoring write errors
func LogWithFallback(msg string) {
	// Ignored: fmt.Fprintf can fail on broken pipes
	_, _ = fmt.Fprintf(nil, "%s\n", msg) // Would be os.Stderr in real code
}
