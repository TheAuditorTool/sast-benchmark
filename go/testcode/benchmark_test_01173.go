package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest01173(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	from := r.FormValue("from")
	to := r.FormValue("to")
	amount, err := strconv.ParseFloat(r.FormValue("amount"), 64)
	if err != nil {
		http.Error(w, "invalid amount", http.StatusBadRequest)
		return
	}
	_, err = DB.Exec("INSERT INTO transfers (from_account, to_account, amount) VALUES (?, ?, ?)", from, to, amount)
	if err != nil {
		http.Error(w, "transfer failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "transferred"})
}
