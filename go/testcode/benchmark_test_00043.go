package testcode

import (
	"net/http"
)

func BenchmarkTest00043(w http.ResponseWriter, r *http.Request) {
	query := "SELECT " + "id, name, email" + " FROM " + "users"
	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
