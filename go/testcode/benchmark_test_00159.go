package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
	"strings"
	"time"
)

func BenchmarkTest00159(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("user_id")
	if userID == "" {
		http.Error(w, "user_id required", http.StatusBadRequest)
		return
	}

	rand.Seed(time.Now().UnixNano())
	digits := "0123456789"
	indices := rand.Perm(10)
	selected := indices[:6]

	var otpBuilder strings.Builder
	for _, idx := range selected {
		otpBuilder.WriteByte(digits[idx])
	}
	otp := otpBuilder.String()

	_, err := DB.Exec("INSERT INTO otp_codes (user_id, code, expires_at) VALUES (?, ?, ?)",
		userID, otp, time.Now().Add(5*time.Minute).Unix())
	if err != nil {
		http.Error(w, "otp generation failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"message": fmt.Sprintf("OTP sent to user %s", userID),
		"otp":     otp,
	})
}
