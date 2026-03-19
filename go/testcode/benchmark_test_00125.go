package testcode

import (
	"net/http"
)

func BenchmarkTest00125(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("note")
	w.Header().Set("Content-Type", "text/plain")
	w.Write([]byte(userInput))
}
