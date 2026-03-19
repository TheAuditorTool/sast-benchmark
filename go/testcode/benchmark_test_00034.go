package testcode

import (
	"net/http"
)

func BenchmarkTest00034(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	tx, err := DB.Begin()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	stmt, err := tx.Prepare("DELETE FROM users WHERE id = ?")
	if err != nil {
		tx.Rollback()
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer stmt.Close()
	_, err = stmt.Exec(id)
	if err != nil {
		tx.Rollback()
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	tx.Commit()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "deleted"})
}
