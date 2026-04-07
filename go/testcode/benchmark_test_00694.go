package testcode

import (
	"net/http"
)

func BenchmarkTest00694(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("user_id")
	resultCh := make(chan error, 1)

	go func() {
		_, err := DB.Exec("UPDATE sessions SET last_seen = NOW() WHERE user_id = ?", userID)
		resultCh <- err
	}()

	if err := <-resultCh; err != nil {
		http.Error(w, "update error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
