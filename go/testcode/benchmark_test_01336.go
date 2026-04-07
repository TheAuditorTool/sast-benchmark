package testcode

import (
	"encoding/base64"
	"fmt"
	"net/http"
	"strings"
)

func BenchmarkTest01336(w http.ResponseWriter, r *http.Request) {
	token := r.Header.Get("Authorization")
	parts := strings.Split(token, ".")
	if len(parts) != 3 {
		http.Error(w, "invalid token", http.StatusUnauthorized)
		return
	}
	payload, err := base64.RawURLEncoding.DecodeString(parts[1])
	if err != nil {
		fmt.Fprintf(w, "token info: %s", parts[1])
		return
	}
	fmt.Fprintf(w, "token info: %s", payload)
}
