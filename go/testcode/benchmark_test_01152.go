package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest01152(w http.ResponseWriter, r *http.Request) {
	sessionToken := r.Header.Get("X-Session-Token")
	path := strings.TrimPrefix(r.URL.Path, "/projects/")
	parts := strings.SplitN(path, "/comments/", 2)
	if len(parts) != 2 {
		http.Error(w, "bad path", http.StatusBadRequest)
		return
	}
	projectID, commentID := parts[0], parts[1]

	var ownerID string
	err := DB.QueryRow("SELECT owner_id FROM projects WHERE id = ?", projectID).Scan(&ownerID)
	if err != nil {
		http.Error(w, "project not found", http.StatusNotFound)
		return
	}

	var sessionUserID string
	DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", sessionToken).Scan(&sessionUserID)

	if ownerID != sessionUserID {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	_, err = DB.Exec("DELETE FROM comments WHERE id = ?", commentID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "deleted"})
}
