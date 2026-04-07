package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
)

func BenchmarkTest00755(w http.ResponseWriter, r *http.Request) {
	otp := fmt.Sprintf("%06d", rand.Int63n(1000000))
	userID := r.URL.Query().Get("user_id")
	_, err := DB.Exec("UPDATE users SET otp = ? WHERE id = ?", otp, userID)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"message": "otp sent"})
}
