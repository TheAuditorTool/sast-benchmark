package testcode

import (
	"net/http"
)

func BenchmarkTest00006(w http.ResponseWriter, r *http.Request) {
	searchTerm := r.URL.Query().Get("q")
	rows, err := DB.Query("SELECT * FROM products WHERE name LIKE ?", "%"+searchTerm+"%")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "searched"})
}
