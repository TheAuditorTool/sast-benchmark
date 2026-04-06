package testcode

import (
	"net/http"
	"runtime"
)

func BenchmarkTest00672(w http.ResponseWriter, r *http.Request) {
	defer func() {
		if rec := recover(); rec != nil {
			buf := make([]byte, 4096)
			n := runtime.Stack(buf, false)
			w.WriteHeader(http.StatusInternalServerError)
			w.Write(buf[:n])
		}
	}()

	orderID := r.URL.Query().Get("order_id")
	if orderID == "" {
		http.Error(w, "order_id required", http.StatusBadRequest)
		return
	}

	var status string
	var total float64
	err := DB.QueryRowContext(r.Context(),
		"SELECT status, total FROM orders WHERE id = ?", orderID,
	).Scan(&status, &total)
	if err != nil {
		http.Error(w, "order not found", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"order_id": orderID,
		"status":   status,
		"total":    total,
	})
}
