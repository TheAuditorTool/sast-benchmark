package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00030(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var name string
	DB.QueryRow(fmt.Sprintf("SELECT name FROM users WHERE id = %s", id)).Scan(&name)
	query := fmt.Sprintf("SELECT * FROM orders WHERE customer_name = '%s'", name)
	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
