package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00033(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	tx, err := DB.Begin()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	query := fmt.Sprintf("DELETE FROM users WHERE id = %s", id)
	_, err = tx.Exec(query)
	if err != nil {
		tx.Rollback()
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	tx.Commit()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "deleted"})
}
