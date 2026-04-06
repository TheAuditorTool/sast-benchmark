package testcode

import (
	"net/http"
	"runtime"
)

func BenchmarkTest00685(w http.ResponseWriter, r *http.Request) {
	sessionToken := r.Header.Get("X-Session-Token")
	if sessionToken == "" {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var role string
	err := DB.QueryRowContext(r.Context(),
		"SELECT role FROM sessions WHERE token = ?", sessionToken,
	).Scan(&role)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	if role != "admin" {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	var memStats runtime.MemStats
	runtime.ReadMemStats(&memStats)

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"go_version":      runtime.Version(),
		"goroutine_count": runtime.NumGoroutine(),
		"heap_alloc":      memStats.HeapAlloc,
		"heap_sys":        memStats.HeapSys,
		"num_gc":          memStats.NumGC,
	})
}
