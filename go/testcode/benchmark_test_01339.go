package testcode

import (
	"log/slog"
	"net/http"
)

func BenchmarkTest01339(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var total float64
	err := DB.QueryRow("SELECT SUM(amount) FROM orders WHERE user_id = ?", id).Scan(&total)
	if err != nil {
		slog.Error("query failed", "err", err)
		w.WriteHeader(http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]float64{"total": total})
}
