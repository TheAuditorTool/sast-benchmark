package testcode

import (
	"net/http"
)

func BenchmarkTest00274(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("name")
	var items []string
	items = append(items, param)
	rows, err := DB.Query("SELECT * FROM users WHERE name = '" + items[0] + "'")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
