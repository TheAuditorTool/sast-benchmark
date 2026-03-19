package testcode

import (
	"net/http"
)

func BenchmarkTest00039(w http.ResponseWriter, r *http.Request) {
	a := r.URL.Query().Get("name")
	b := r.URL.Query().Get("email")
	c := r.URL.Query().Get("role")
	rows, err := DB.Query(
		"SELECT * FROM users WHERE name = ? AND email = ? AND role = ?",
		a, b, c,
	)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
