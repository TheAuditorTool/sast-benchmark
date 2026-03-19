package testcode

import (
	"net/http"
)

type SafeOrderResult struct {
	UserID  uint
	OrderID uint
}

func BenchmarkTest00392(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("id")
	var results []SafeOrderResult
	GormDB.Joins("JOIN orders ON users.id = orders.user_id AND users.id = ?", userID).Find(&results)
	RespondJSON(w, http.StatusOK, results)
}
