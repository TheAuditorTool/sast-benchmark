package testcode

import (
	"log/slog"
	"net/http"
)

func BenchmarkTest00679(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("user_id")
	if userID == "" {
		http.Error(w, "user_id required", http.StatusBadRequest)
		return
	}

	var balance float64
	var currency string
	err := DB.QueryRowContext(r.Context(),
		"SELECT balance, currency FROM accounts WHERE user_id = ?", userID,
	).Scan(&balance, &currency)
	if err != nil {
		slog.Error("database query failed", "error", err, "user_id", userID)
		http.Error(w, "internal server error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"balance":  balance,
		"currency": currency,
	})
}
