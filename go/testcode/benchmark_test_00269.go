package testcode

import (
	"net/http"
)

func BenchmarkTest00269(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("id")
	m := make(map[string]string)
	m["tainted"] = param
	bar := m["clean"]
	rows, err := DB.Query("SELECT * FROM users WHERE id = '" + bar + "'")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
