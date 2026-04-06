package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest00613(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("session")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var callerID int
	err = DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", cookie.Value).Scan(&callerID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	targetIDStr := r.URL.Query().Get("target_id")
	targetID, err := strconv.Atoi(targetIDStr)
	if err != nil {
		http.Error(w, "invalid target_id", http.StatusBadRequest)
		return
	}

	result, err := DB.Exec("DELETE FROM users WHERE id = ?", targetID)
	if err != nil {
		http.Error(w, "delete failed", http.StatusInternalServerError)
		return
	}

	affected, _ := result.RowsAffected()
	if affected == 0 {
		http.Error(w, "user not found", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"deleted": targetID,
	})
}
