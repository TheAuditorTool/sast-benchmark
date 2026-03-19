package testcode

import (
	"encoding/json"
	"net/http"
)

func BenchmarkTest00123(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("search")
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"query": userInput})
}
