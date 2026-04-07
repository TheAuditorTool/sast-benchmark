package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest01163(w http.ResponseWriter, r *http.Request) {
	sessionToken := r.Header.Get("X-Session-Token")
	itemID := strings.TrimPrefix(r.URL.Path, "/items/")

	var id, name, description string
	err := DB.QueryRow(`
		SELECT i.id, i.name, i.description
		FROM items i
		JOIN users u ON i.owner_id = u.id
		WHERE u.session_token = ? AND i.id = ?
	`, sessionToken, itemID).Scan(&id, &name, &description)
	if err != nil {
		http.Error(w, "not found or forbidden", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"id": id, "name": name, "description": description,
	})
}
