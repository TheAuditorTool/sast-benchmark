package testcode

import (
	"net/http"
)

func BenchmarkTest00021(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("search")
	queryType := r.URL.Query().Get("type")
	var query string
	var arg string
	switch queryType {
	case "name":
		query = "SELECT * FROM users WHERE name = ?"
		arg = input
	case "email":
		query = "SELECT * FROM users WHERE email = ?"
		arg = input
	default:
		query = "SELECT * FROM users WHERE id = ?"
		arg = input
	}
	rows, err := DB.Query(query, arg)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
