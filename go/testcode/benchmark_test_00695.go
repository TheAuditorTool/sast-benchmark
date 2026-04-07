package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest00695(w http.ResponseWriter, r *http.Request) {
	table := "users"
	userID := r.URL.Query().Get("id")

	query := strings.Replace("SELECT * FROM __TABLE__ WHERE id = ?", "__TABLE__", table, 1)
	row := DB.QueryRow(query, userID)

	var name string
	if err := row.Scan(&name); err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"name": name})
}
