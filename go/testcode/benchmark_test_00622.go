package testcode

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"net/http"
	"os"
	"strconv"
	"time"
)

func BenchmarkTest00622(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
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

	if err := r.ParseForm(); err != nil {
		http.Error(w, "form parse failed", http.StatusBadRequest)
		return
	}

	submittedToken := r.FormValue("csrf_token")
	timestampStr := r.FormValue("csrf_ts")
	if submittedToken == "" || timestampStr == "" {
		http.Error(w, "missing csrf fields", http.StatusForbidden)
		return
	}

	ts, err := strconv.ParseInt(timestampStr, 10, 64)
	if err != nil || time.Now().Unix()-ts > 3600 {
		http.Error(w, "csrf token expired", http.StatusForbidden)
		return
	}

	csrfKey := []byte(os.Getenv("CSRF_KEY"))
	mac := hmac.New(sha256.New, csrfKey)
	mac.Write([]byte(fmt.Sprintf("%s:%d", cookie.Value, ts)))
	expectedToken := hex.EncodeToString(mac.Sum(nil))

	if !hmac.Equal([]byte(submittedToken), []byte(expectedToken)) {
		http.Error(w, "invalid csrf token", http.StatusForbidden)
		return
	}

	var req struct {
		ToAccountID int     `json:"to_account_id"`
		Amount      float64 `json:"amount"`
	}
	toAccountIDStr := r.FormValue("to_account_id")
	amountStr := r.FormValue("amount")
	req.ToAccountID, _ = strconv.Atoi(toAccountIDStr)
	req.Amount, _ = strconv.ParseFloat(amountStr, 64)

	if req.Amount <= 0 || req.ToAccountID == 0 {
		http.Error(w, "invalid transfer parameters", http.StatusBadRequest)
		return
	}

	_, err = DB.Exec(
		"INSERT INTO transfers (from_user_id, to_account_id, amount, created_at) VALUES (?, ?, ?, ?)",
		userID, req.ToAccountID, req.Amount, time.Now().Unix(),
	)
	if err != nil {
		http.Error(w, "transfer failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"status": "transfer initiated",
		"amount": req.Amount,
	})
}
