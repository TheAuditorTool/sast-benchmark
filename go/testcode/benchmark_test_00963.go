package testcode

import (
	"log"
	"net/http"
)

func BenchmarkTest00963(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	action := r.URL.Query().Get("action")
	log.Printf("user=" + username + " action=" + action)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "logged"})
}
