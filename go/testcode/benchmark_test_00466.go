package testcode

import (
	"net/http"
)

type benchmarkTest00466UpdateRequest struct {
	DisplayName string `json:"display_name"`
	Bio         string `json:"bio"`
	Website     string `json:"website"`
}

func BenchmarkTest00466(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	cookie, err := r.Cookie("session_id")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID string
	err = DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", cookie.Value).Scan(&userID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var req benchmarkTest00466UpdateRequest
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	_, err = DB.Exec(
		"UPDATE user_profiles SET display_name = ?, bio = ?, website = ? WHERE user_id = ?",
		req.DisplayName, req.Bio, req.Website, userID,
	)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "profile updated"})
}
