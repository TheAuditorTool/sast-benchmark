package testcode

import (
	"net/http"
	"time"
)

var benchmarkTest00661Ready bool

func BenchmarkTest00661(w http.ResponseWriter, r *http.Request) {
	taskID := r.URL.Query().Get("task_id")
	if taskID == "" {
		http.Error(w, "task_id required", http.StatusBadRequest)
		return
	}

	go func() {
		time.Sleep(10 * time.Millisecond)
		benchmarkTest00661Ready = true
	}()

	time.Sleep(5 * time.Millisecond)

	if benchmarkTest00661Ready {
		RespondJSON(w, http.StatusOK, map[string]interface{}{
			"task_id": taskID,
			"status":  "ready",
		})
		return
	}

	RespondJSON(w, http.StatusAccepted, map[string]interface{}{
		"task_id": taskID,
		"status":  "pending",
	})
}
