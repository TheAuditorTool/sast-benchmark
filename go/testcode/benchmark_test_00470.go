package testcode

import (
	"net/http"
)

func BenchmarkTest00470(w http.ResponseWriter, r *http.Request) {
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

	rows, err := DB.Query(
		"SELECT id, to_account, amount, created_at FROM transfers WHERE from_user_id = ? ORDER BY created_at DESC LIMIT 50",
		userID,
	)
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	var transfers []map[string]interface{}
	for rows.Next() {
		var id, toAccount, amount, createdAt string
		if err := rows.Scan(&id, &toAccount, &amount, &createdAt); err != nil {
			continue
		}
		transfers = append(transfers, map[string]interface{}{
			"id":         id,
			"to_account": toAccount,
			"amount":     amount,
			"created_at": createdAt,
		})
	}

	if transfers == nil {
		transfers = []map[string]interface{}{}
	}

	RespondJSON(w, http.StatusOK, transfers)
}
