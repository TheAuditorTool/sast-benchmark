package testcode

import (
	"log/slog"
	"net/http"
	"runtime"

	"github.com/google/uuid"
)

func BenchmarkTest00682(w http.ResponseWriter, r *http.Request) {
	reqID := uuid.New().String()

	defer func() {
		if rec := recover(); rec != nil {
			buf := make([]byte, 4096)
			n := runtime.Stack(buf, false)
			slog.Error("panic recovered",
				"error", rec,
				"request_id", reqID,
				"stack", string(buf[:n]),
			)
			RespondJSON(w, http.StatusInternalServerError, map[string]string{
				"error":      "internal error",
				"request_id": reqID,
			})
		}
	}()

	itemID := r.URL.Query().Get("item_id")
	if itemID == "" {
		http.Error(w, "item_id required", http.StatusBadRequest)
		return
	}

	var name string
	var stock int
	err := DB.QueryRowContext(r.Context(),
		"SELECT name, stock FROM inventory WHERE id = ?", itemID,
	).Scan(&name, &stock)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"request_id": reqID,
		"name":       name,
		"stock":      stock,
	})
}
