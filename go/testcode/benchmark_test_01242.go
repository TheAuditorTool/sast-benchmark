package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest01242(w http.ResponseWriter, r *http.Request) {
	uid := r.URL.Query().Get("uid")
	amount, _ := strconv.Atoi(r.URL.Query().Get("amount"))
	var balance int
	DB.QueryRow("SELECT balance FROM accounts WHERE uid = ?", uid).Scan(&balance)
	if balance < amount {
		http.Error(w, "insufficient funds", http.StatusBadRequest)
		return
	}
	DB.Exec("UPDATE accounts SET balance = balance - ? WHERE uid = ?", amount, uid)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "debited"})
}
