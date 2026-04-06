package testcode

import (
	"net/http"
	"time"
)

var benchmarkTest00666Sem = make(chan struct{}, 10)

func BenchmarkTest00666(w http.ResponseWriter, r *http.Request) {
	jobID := r.URL.Query().Get("job_id")
	if jobID == "" {
		http.Error(w, "job_id required", http.StatusBadRequest)
		return
	}

	select {
	case benchmarkTest00666Sem <- struct{}{}:
	case <-time.After(5 * time.Second):
		http.Error(w, "service busy", http.StatusServiceUnavailable)
		return
	}
	defer func() { <-benchmarkTest00666Sem }()

	var result struct {
		ID   string `json:"id"`
		Name string `json:"name"`
	}
	err := DB.QueryRow("SELECT id, name FROM jobs WHERE id = ?", jobID).Scan(&result.ID, &result.Name)
	if err != nil {
		http.Error(w, "job not found", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, result)
}
