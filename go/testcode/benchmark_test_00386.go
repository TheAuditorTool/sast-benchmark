package testcode

import (
	"net/http"
)

func BenchmarkTest00386(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	defer func() {
		DB.Exec("INSERT INTO audit (user_id) VALUES (?)", id)
	}()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
