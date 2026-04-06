package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest00667(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("user_id")
	amountStr := r.URL.Query().Get("amount")

	amount, err := strconv.ParseFloat(amountStr, 64)
	if err != nil || amount <= 0 {
		http.Error(w, "invalid amount", http.StatusBadRequest)
		return
	}

	tx, err := DB.Begin()
	if err != nil {
		http.Error(w, "transaction error", http.StatusInternalServerError)
		return
	}
	defer tx.Rollback()

	var balance float64
	err = tx.QueryRow("SELECT balance FROM accounts WHERE id = ? FOR UPDATE", userID).Scan(&balance)
	if err != nil {
		http.Error(w, "account not found", http.StatusNotFound)
		return
	}

	if balance < amount {
		http.Error(w, "insufficient funds", http.StatusPaymentRequired)
		return
	}

	_, err = tx.Exec("UPDATE accounts SET balance = balance - ? WHERE id = ?", amount, userID)
	if err != nil {
		http.Error(w, "deduction failed", http.StatusInternalServerError)
		return
	}

	if err := tx.Commit(); err != nil {
		http.Error(w, "commit failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"user_id":           userID,
		"deducted":          amount,
		"remaining_balance": balance - amount,
	})
}
