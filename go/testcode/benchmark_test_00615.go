package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest00615(w http.ResponseWriter, r *http.Request) {
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

	orderIDStr := r.URL.Query().Get("order_id")
	orderID, err := strconv.Atoi(orderIDStr)
	if err != nil {
		http.Error(w, "invalid order_id", http.StatusBadRequest)
		return
	}

	var order struct {
		ID        int
		UserID    int
		Total     float64
		Status    string
		CreatedAt string
	}

	err = DB.QueryRow(
		`SELECT o.id, o.user_id, o.total, o.status, o.created_at
		 FROM orders o
		 INNER JOIN users u ON o.user_id = u.id
		 WHERE o.id = ? AND u.id = ?`,
		orderID, userID,
	).Scan(&order.ID, &order.UserID, &order.Total, &order.Status, &order.CreatedAt)
	if err != nil {
		http.Error(w, "order not found", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"id":         order.ID,
		"user_id":    order.UserID,
		"total":      order.Total,
		"status":     order.Status,
		"created_at": order.CreatedAt,
	})
}
