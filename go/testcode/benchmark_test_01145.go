package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest01145(w http.ResponseWriter, r *http.Request) {
	targetUserID := strings.TrimPrefix(r.URL.Path, "/users/")
	targetUserID = strings.TrimSuffix(targetUserID, "/email")

	var body struct {
		Email string `json:"email"`
	}
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	_, err := DB.Exec("UPDATE users SET email = ? WHERE id = ?", body.Email, targetUserID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}
