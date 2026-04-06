package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00670(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	if id == "" {
		http.Error(w, "id required", http.StatusBadRequest)
		return
	}

	var username string
	row := DB.QueryRowContext(r.Context(),
		"SELECT username FROM users WHERE id = ?", id,
	)
	err := row.Scan(&username)
	if err != nil {
		wrapped := fmt.Errorf("query user %s: %w", id, err)
		http.Error(w, fmt.Sprintf("Error: %+v", wrapped), http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"username": username})
}
