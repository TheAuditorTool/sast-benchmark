package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest01155(w http.ResponseWriter, r *http.Request) {
	sessionToken := r.Header.Get("X-Session-Token")
	docID := strings.TrimPrefix(r.URL.Path, "/documents/")

	var sessionUserID string
	err := DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", sessionToken).Scan(&sessionUserID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var creatorID string
	err = DB.QueryRow("SELECT creator_id FROM documents WHERE id = ?", docID).Scan(&creatorID)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}

	if creatorID != sessionUserID {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	_, err = DB.Exec("UPDATE documents SET content = 'deleted' WHERE id = ?", docID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}
