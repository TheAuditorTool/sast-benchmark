package testcode

import (
	"net/http"
	"strings"
)

var benchmarkTest00624ExpectedOrigin = "https://example.com"

func BenchmarkTest00624(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	referer := r.Header.Get("Referer")
	if referer == "" || !strings.HasPrefix(referer, benchmarkTest00624ExpectedOrigin) {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	cookie, err := r.Cookie("session")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID int
	err = DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", cookie.Value).Scan(&userID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var req struct {
		FirstName string `json:"first_name"`
		LastName  string `json:"last_name"`
		Phone     string `json:"phone"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	_, err = DB.Exec(
		"UPDATE users SET first_name = ?, last_name = ?, phone = ? WHERE id = ?",
		req.FirstName, req.LastName, req.Phone, userID,
	)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "profile updated"})
}
