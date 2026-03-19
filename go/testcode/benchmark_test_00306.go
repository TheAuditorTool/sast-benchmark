package testcode

import (
	"net/http"

	"github.com/go-chi/chi/v5"
)

func BenchmarkTest00306(w http.ResponseWriter, r *http.Request) {
	id := chi.URLParam(r, "id")
	rows, err := DB.Query("SELECT * FROM users WHERE id = ?", id)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
