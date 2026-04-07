package testcode

import (
	"net/http"

	"golang.org/x/crypto/bcrypt"
)

func BenchmarkTest01138(w http.ResponseWriter, r *http.Request) {
	step := r.URL.Query().Get("step")

	if step == "1" {
		username := r.FormValue("username")
		password := r.FormValue("password")

		var hashedPassword string
		var userID int
		row := DB.QueryRow("SELECT id, password FROM users WHERE username = ?", username)
		if err := row.Scan(&userID, &hashedPassword); err != nil {
			http.Error(w, "invalid credentials", http.StatusUnauthorized)
			return
		}

		if err := bcrypt.CompareHashAndPassword([]byte(hashedPassword), []byte(password)); err != nil {
			http.Error(w, "invalid credentials", http.StatusUnauthorized)
			return
		}

		DB.Exec(
			"INSERT INTO mfa_pending (user_id, expires_at) VALUES (?, datetime('now', '+5 minutes'))",
			userID,
		)
		RespondJSON(w, http.StatusOK, map[string]string{"status": "awaiting_mfa"})
		return
	}

	otp := r.FormValue("otp")
	mfaToken := r.Header.Get("X-MFA-Token")

	var userID int
	row := DB.QueryRow(
		"SELECT user_id FROM mfa_pending WHERE token = ? AND expires_at > datetime('now')",
		mfaToken,
	)
	if err := row.Scan(&userID); err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var storedOTP string
	row = DB.QueryRow("SELECT otp FROM totp_secrets WHERE user_id = ?", userID)
	if err := row.Scan(&storedOTP); err != nil || storedOTP != otp {
		http.Error(w, "invalid OTP", http.StatusUnauthorized)
		return
	}

	DB.Exec("DELETE FROM mfa_pending WHERE token = ?", mfaToken)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "authenticated"})
}
