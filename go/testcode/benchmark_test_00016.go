package testcode

import (
	"net/http"
)

func BenchmarkTest00016(w http.ResponseWriter, r *http.Request) {
	userAgent := r.Header.Get("User-Agent")
	referer := r.Header.Get("Referer")
	_, err := DB.Exec(
		"INSERT INTO access_logs (user_agent, referer) VALUES (?, ?)",
		userAgent, referer,
	)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "logged"})
}
