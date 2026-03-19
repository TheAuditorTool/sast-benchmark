package testcode

import (
	"net/http"
)

func BenchmarkTest00380(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var name string
	err := DB.QueryRow("SELECT name FROM users WHERE id = ?", id).Scan(&name)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	DB.Exec("INSERT INTO audit (name) VALUES (?)", name)
	RespondJSON(w, http.StatusOK, map[string]string{"name": name})
}
