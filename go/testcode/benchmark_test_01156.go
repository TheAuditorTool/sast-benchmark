package testcode

import (
	"net/http"
)

func BenchmarkTest01156(w http.ResponseWriter, r *http.Request) {
	isAdmin := r.URL.Query().Get("admin") == "true"
	if !isAdmin {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	var body struct {
		UserID string `json:"user_id"`
		Role   string `json:"role"`
	}
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	_, err := DB.Exec("UPDATE users SET role = ? WHERE id = ?", body.Role, body.UserID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "role updated"})
}
