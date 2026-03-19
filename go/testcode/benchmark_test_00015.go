package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00015(w http.ResponseWriter, r *http.Request) {
	userAgent := r.Header.Get("User-Agent")
	referer := r.Header.Get("Referer")
	query := fmt.Sprintf(
		"INSERT INTO access_logs (user_agent, referer) VALUES ('%s', '%s')",
		userAgent, referer,
	)
	_, err := DB.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "logged"})
}
