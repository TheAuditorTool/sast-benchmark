package testcode

import (
	"log"
	"net/http"
	"strings"
)

func BenchmarkTest00981(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	safe := strings.ReplaceAll(username, "\n", "\\n")
	safe = strings.ReplaceAll(safe, "\r", "\\r")
	log.Printf("user=%s login", safe)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
