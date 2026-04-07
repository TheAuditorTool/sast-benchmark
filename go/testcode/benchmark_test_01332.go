package testcode

import (
	"net/http"
	"runtime/debug"
)

func BenchmarkTest01332(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var name string
	err := DB.QueryRow("SELECT name FROM users WHERE id = ?", id).Scan(&name)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write(debug.Stack())
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"name": name})
}
