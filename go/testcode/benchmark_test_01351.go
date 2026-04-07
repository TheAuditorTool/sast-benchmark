package testcode

import (
	"database/sql"
	"errors"
	"net/http"
)

func BenchmarkTest01351(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var name string
	err := DB.QueryRow("SELECT name FROM users WHERE id = ?", id).Scan(&name)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			http.Error(w, "not found", http.StatusNotFound)
		} else {
			http.Error(w, "error", http.StatusInternalServerError)
		}
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"name": name})
}
