package testcode

import (
	"log"
	"net/http"
	"strings"
)

func BenchmarkTest00533(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")

	sanitized := strings.ReplaceAll(username, "\n", "")
	sanitized = strings.ReplaceAll(sanitized, "\r", "")

	log.Printf("login attempt: user=%s", sanitized)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
