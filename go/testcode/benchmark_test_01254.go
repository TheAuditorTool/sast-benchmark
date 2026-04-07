package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest01254(w http.ResponseWriter, r *http.Request) {
	uid := r.URL.Query().Get("uid")
	amount, _ := strconv.Atoi(r.URL.Query().Get("amount"))
	tx, err := DB.Begin()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer tx.Rollback()
	var balance int
	tx.QueryRow("SELECT balance FROM accounts WHERE uid = ? FOR UPDATE", uid).Scan(&balance)
	if balance < amount {
		http.Error(w, "insufficient funds", http.StatusBadRequest)
		return
	}
	tx.Exec("UPDATE accounts SET balance = balance - ? WHERE uid = ?", amount, uid)
	tx.Commit()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "debited"})
}
