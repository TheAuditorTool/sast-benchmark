package testcode

import (
	"log"
	"net/http"
)

func BenchmarkTest00965(w http.ResponseWriter, r *http.Request) {
	query := r.URL.Query().Get("q")
	log.Println("search query:", query)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
