package testcode

import (
	"net/http"
)

func BenchmarkTest00290(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("id")
	buildQuery := func() string {
		return "SELECT * FROM users WHERE id = '" + param + "'"
	}
	rows, err := DB.Query(buildQuery())
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
