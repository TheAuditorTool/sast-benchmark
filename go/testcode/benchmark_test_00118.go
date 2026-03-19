package testcode

import (
	"encoding/json"
	"net/http"
)

func BenchmarkTest00118(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("data")
	w.Header().Set("Content-Type", "text/html")
	json.NewEncoder(w).Encode(map[string]string{"value": userInput})
}
