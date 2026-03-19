package testcode

import (
	"fmt"
	"net/http"

	"github.com/go-chi/chi/v5"
)

func BenchmarkTest00303(w http.ResponseWriter, r *http.Request) {
	id := chi.URLParam(r, "id")
	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", id)
	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
