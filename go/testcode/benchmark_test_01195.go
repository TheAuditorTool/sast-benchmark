package testcode

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"net/http"
	"strconv"
	"strings"
	"time"
)

var benchmarkTest01195Secret = []byte("hmac-expiry-csrf-secret-key")

func benchmarkTest01195Verify(token string) bool {
	parts := strings.SplitN(token, ".", 2)
	if len(parts) != 2 {
		return false
	}
	ts, err := strconv.ParseInt(parts[0], 10, 64)
	if err != nil || time.Now().Unix()-ts > 3600 {
		return false
	}
	mac := hmac.New(sha256.New, benchmarkTest01195Secret)
	mac.Write([]byte(fmt.Sprintf("%d", ts)))
	expected := hex.EncodeToString(mac.Sum(nil))
	return hmac.Equal([]byte(parts[1]), []byte(expected))
}

func BenchmarkTest01195(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	token := r.FormValue("csrf_token")
	if !benchmarkTest01195Verify(token) {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	userID := r.FormValue("user_id")
	newPhone := r.FormValue("phone")
	_, err := DB.Exec("UPDATE users SET phone = ? WHERE id = ?", newPhone, userID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "phone updated"})
}
