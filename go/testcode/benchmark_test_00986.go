package testcode

import (
	"crypto/sha256"
	"log"
	"net/http"
)

func BenchmarkTest00986(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	h := sha256.Sum256([]byte(username))
	log.Printf("login attempt hash=%x", h)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
