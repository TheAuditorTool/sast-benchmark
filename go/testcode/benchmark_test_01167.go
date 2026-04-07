package testcode

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"net/http"
	"os"
	"strings"
)

var benchmarkTest01167HMACSecret = os.Getenv("RESOURCE_HMAC_SECRET")

func BenchmarkTest01167(w http.ResponseWriter, r *http.Request) {
	resourceToken := r.Header.Get("X-Resource-Token")
	resourceID := strings.TrimPrefix(r.URL.Path, "/resources/")

	parts := strings.SplitN(resourceToken, ":", 3)
	if len(parts) != 3 {
		http.Error(w, "invalid token", http.StatusUnauthorized)
		return
	}
	tokenUserID, tokenResourceID, sig := parts[0], parts[1], parts[2]

	mac := hmac.New(sha256.New, []byte(benchmarkTest01167HMACSecret))
	mac.Write([]byte(fmt.Sprintf("%s:%s", tokenUserID, tokenResourceID)))
	expected := hex.EncodeToString(mac.Sum(nil))

	if !hmac.Equal([]byte(sig), []byte(expected)) || tokenResourceID != resourceID {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	var data string
	err := DB.QueryRow("SELECT data FROM resources WHERE id = ?", resourceID).Scan(&data)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"data": data})
}
