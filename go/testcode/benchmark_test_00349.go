package testcode

import (
	"log"
	"net/http"
)

func BenchmarkTest00349(w http.ResponseWriter, r *http.Request) {
	count := len(r.URL.Query())
	log.Printf("Param count: %d", count)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
