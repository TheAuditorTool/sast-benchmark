package testcode

import (
	"net/http"
)

func BenchmarkTest01183(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var body struct {
		UserID  string `json:"user_id"`
		Message string `json:"message"`
	}
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "invalid json", http.StatusBadRequest)
		return
	}
	_, err := DB.Exec("INSERT INTO messages (user_id, content) VALUES (?, ?)", body.UserID, body.Message)
	if err != nil {
		http.Error(w, "insert failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "message sent"})
}
