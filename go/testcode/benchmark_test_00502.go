package testcode

import (
	"net/http"
)

func BenchmarkTest00502(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	cookie, err := r.Cookie("session_id")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID, teamID string
	err = DB.QueryRow("SELECT user_id, team_id FROM sessions WHERE token = ?", cookie.Value).Scan(&userID, &teamID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	inviteEmail := r.FormValue("email")
	role := r.FormValue("role")
	if inviteEmail == "" {
		http.Error(w, "email required", http.StatusBadRequest)
		return
	}

	_, err = DB.Exec(
		"INSERT INTO team_invitations (team_id, invited_by, email, role) VALUES (?, ?, ?, ?)",
		teamID, userID, inviteEmail, role,
	)
	if err != nil {
		http.Error(w, "invite failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "invite_sent", "email": inviteEmail})
}
