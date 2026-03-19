package testcode

import (
	"net/http"
)

type OrderResult struct {
	UserID  uint
	OrderID uint
}

func BenchmarkTest00391(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("id")
	var results []OrderResult
	GormDB.Joins("JOIN orders ON users.id = orders.user_id AND users.id = " + userID).Find(&results)
	RespondJSON(w, http.StatusOK, results)
}
