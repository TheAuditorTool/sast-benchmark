package testcode

import (
	"net/http"
)

func BenchmarkTest00041(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	_, err := DB.Exec("DELETE FROM users WHERE id = ?", id)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "deleted"})
}
