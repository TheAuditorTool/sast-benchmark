package testcode

import (
	"net/http"
)

func BenchmarkTest00640(w http.ResponseWriter, r *http.Request) {
	var body map[string]interface{}
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "invalid JSON body", http.StatusBadRequest)
		return
	}

	userID := body["user_id"].(string)
	action := body["action"].(string)

	_, err := DB.Exec(
		"INSERT INTO audit_log (user_id, action) VALUES (?, ?)",
		userID, action,
	)
	if err != nil {
		http.Error(w, "database error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"status":  "logged",
		"user_id": userID,
		"action":  action,
	})
}
