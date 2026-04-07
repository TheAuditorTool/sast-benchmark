package testcode

import (
	"log"
	"net/http"
)

func BenchmarkTest00976(w http.ResponseWriter, r *http.Request) {
	ip := r.Header.Get("X-Forwarded-For")
	path := r.URL.Path
	log.Printf("request from %s to %s", ip, path)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
