package testcode

import (
	"fmt"
	"log/slog"
	"net/http"
)

func BenchmarkTest00688(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("user_id")
	if userID == "" {
		http.Error(w, "user_id required", http.StatusBadRequest)
		return
	}

	tableName := "user_preferences"

	var theme string
	var notificationsEnabled bool
	err := DB.QueryRowContext(r.Context(),
		"SELECT theme, notifications_enabled FROM user_preferences WHERE user_id = ?", userID,
	).Scan(&theme, &notificationsEnabled)
	if err != nil {
		internalErr := fmt.Errorf("user %s query on table %s failed: %w", userID, tableName, err)
		slog.Error("operation failed", "error", internalErr)
		RespondJSON(w, http.StatusInternalServerError, map[string]string{
			"error": "operation failed",
		})
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"theme":                  theme,
		"notifications_enabled":  notificationsEnabled,
	})
}
