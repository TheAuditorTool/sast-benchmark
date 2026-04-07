package testcode

import (
	"log/slog"
	"net/http"
)

func BenchmarkTest01349(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var name string
	err := DB.QueryRow("SELECT name FROM accounts WHERE id = ?", id).Scan(&name)
	if err != nil {
		slog.Error("internal error", "err", err)
		http.Error(w, "something went wrong", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"name": name})
}
