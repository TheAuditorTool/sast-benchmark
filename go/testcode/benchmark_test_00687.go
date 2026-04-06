package testcode

import (
	"net/http"
)

func BenchmarkTest00687(w http.ResponseWriter, r *http.Request) {
	w.Header().Del("Server")

	tagID := r.URL.Query().Get("tag_id")
	if tagID == "" {
		http.Error(w, "tag_id required", http.StatusBadRequest)
		return
	}

	rows, err := DB.QueryContext(r.Context(),
		"SELECT id, title FROM articles WHERE tag_id = ? ORDER BY published_at DESC LIMIT 20", tagID,
	)
	if err != nil {
		http.Error(w, "internal server error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	type article struct {
		ID    int    `json:"id"`
		Title string `json:"title"`
	}
	var articles []article
	for rows.Next() {
		var a article
		if err := rows.Scan(&a.ID, &a.Title); err != nil {
			http.Error(w, "internal server error", http.StatusInternalServerError)
			return
		}
		articles = append(articles, a)
	}

	RespondJSON(w, http.StatusOK, articles)
}
