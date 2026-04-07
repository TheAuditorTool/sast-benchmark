package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest01169(w http.ResponseWriter, r *http.Request) {
	sessionToken := r.Header.Get("X-Session-Token")
	resourceID := strings.TrimPrefix(r.URL.Path, "/resources/")

	var userID string
	err := DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", sessionToken).Scan(&userID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var exists int
	err = DB.QueryRow(
		"SELECT 1 FROM permissions WHERE user_id = ? AND resource_id = ? AND perm = 'write'",
		userID, resourceID,
	).Scan(&exists)
	if err != nil {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	var body struct {
		Data string `json:"data"`
	}
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	DB.Exec("UPDATE resources SET data = ? WHERE id = ?", body.Data, resourceID)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "written"})
}
