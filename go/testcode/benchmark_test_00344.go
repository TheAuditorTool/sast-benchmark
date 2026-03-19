package testcode

import (
	"log"
	"net/http"
)

func BenchmarkTest00344(w http.ResponseWriter, r *http.Request) {
	query := r.URL.Query().Get("q")
	ip := r.Header.Get("X-Forwarded-For")
	log.Printf("Search query: %s from IP: %s", query, ip)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
