package testcode

import (
	"net/http"
)

func BenchmarkTest00461(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("session_id")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var authUserID string
	err = DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", cookie.Value).Scan(&authUserID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	itemID := r.URL.Query().Get("id")
	if itemID == "" {
		http.Error(w, "missing id", http.StatusBadRequest)
		return
	}

	result, err := DB.Exec("DELETE FROM items WHERE id = ? AND owner_id = ?", itemID, authUserID)
	if err != nil {
		http.Error(w, "delete failed", http.StatusInternalServerError)
		return
	}

	n, _ := result.RowsAffected()
	if n == 0 {
		http.Error(w, "not found or forbidden", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "deleted"})
}
