package testcode

import (
	"fmt"
	"net/http"
	"os"
	"runtime"
)

func BenchmarkTest00677(w http.ResponseWriter, r *http.Request) {
	hostname, _ := os.Hostname()

	w.Header().Set("Server", fmt.Sprintf("MyApp/1.0 Go/%s Host/%s", runtime.Version(), hostname))
	w.Header().Set("Content-Type", "text/plain")

	userID := r.URL.Query().Get("user_id")
	if userID == "" {
		http.Error(w, "user_id required", http.StatusBadRequest)
		return
	}

	var name string
	err := DB.QueryRowContext(r.Context(),
		"SELECT name FROM users WHERE id = ?", userID,
	).Scan(&name)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"name": name})
}
