package testcode

import (
	"net/http"
)

func BenchmarkTest00035(w http.ResponseWriter, r *http.Request) {
	name := r.URL.Query().Get("name")
	value := r.URL.Query().Get("value")
	_ = name
	rows, err := DB.Query("SELECT * FROM config WHERE key = 'default_theme'")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	_ = value
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
