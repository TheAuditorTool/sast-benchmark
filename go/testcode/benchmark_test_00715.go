package testcode

import (
	"encoding/json"
	"net/http"
)

func BenchmarkTest00715(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("msg")
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(struct{ Msg string }{Msg: input})
}
