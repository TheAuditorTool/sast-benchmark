package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00020(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("search")
	queryType := r.URL.Query().Get("type")
	var query string
	switch queryType {
	case "name":
		query = fmt.Sprintf("SELECT * FROM users WHERE name = '%s'", input)
	case "email":
		query = fmt.Sprintf("SELECT * FROM users WHERE email = '%s'", input)
	default:
		query = fmt.Sprintf("SELECT * FROM users WHERE id = '%s'", input)
	}
	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
