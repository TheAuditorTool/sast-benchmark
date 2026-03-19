package testcode

import (
	"log"
	"net/http"
)

func BenchmarkTest00350(w http.ResponseWriter, r *http.Request) {
	log.Println("Health check endpoint called")
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
