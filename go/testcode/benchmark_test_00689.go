package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest00689(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	items := r.Form["item"]
	if len(items) == 0 {
		RespondJSON(w, http.StatusBadRequest, map[string]string{"error": "no items"})
		return
	}

	var b strings.Builder
	args := make([]interface{}, len(items))
	placeholders := make([]string, len(items))
	for i, v := range items {
		placeholders[i] = "?"
		args[i] = v
	}
	b.WriteString("SELECT id FROM products WHERE name IN (")
	b.WriteString(strings.Join(placeholders, ","))
	b.WriteString(")")

	rows, err := DB.Query(b.String(), args...)
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
