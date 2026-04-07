package testcode

import (
	"log"
	"net/http"
)

func BenchmarkTest00979(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	log.Printf("user=%q logged in", username)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
