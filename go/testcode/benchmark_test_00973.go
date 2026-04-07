package testcode

import (
	"log"
	"net/http"
)

func BenchmarkTest00973(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("template")
	log.Printf(userInput)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
