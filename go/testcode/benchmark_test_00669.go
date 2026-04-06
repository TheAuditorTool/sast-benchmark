package testcode

import (
	"net/http"
	"runtime/debug"
)

func BenchmarkTest00669(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("user_id")
	if userID == "" {
		http.Error(w, "user_id required", http.StatusBadRequest)
		return
	}

	var email string
	err := DB.QueryRowContext(r.Context(),
		"SELECT email FROM users WHERE id = ?", userID,
	).Scan(&email)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write(debug.Stack())
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"email": email})
}
