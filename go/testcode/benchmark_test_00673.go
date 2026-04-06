package testcode

import (
	"net/http"
)

func BenchmarkTest00673(w http.ResponseWriter, r *http.Request) {
	slug := r.URL.Query().Get("slug")
	if slug == "" {
		http.Error(w, "slug required", http.StatusBadRequest)
		return
	}

	var id int
	var title, body string
	err := DB.QueryRowContext(r.Context(),
		"SELECT id, title, body FROM posts WHERE slug = ?", slug,
	).Scan(&id, &title, &body)
	if err != nil {
		http.Error(w, "Database error: "+err.Error(), http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"id":    id,
		"title": title,
		"body":  body,
	})
}
