package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest00005(w http.ResponseWriter, r *http.Request) {
	searchTerm := r.URL.Query().Get("q")
	var b strings.Builder
	b.WriteString("SELECT * FROM products WHERE name LIKE '%")
	b.WriteString(searchTerm)
	b.WriteString("%'")
	rows, err := DB.Query(b.String())
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "searched"})
}
