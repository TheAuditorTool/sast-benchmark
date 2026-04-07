package testcode

import (
	"encoding/base64"
	"fmt"
	"net/http"
)

const benchmarkTest01097EncodedKey = "c2VjcmV0LWFwaS1rZXktdmFsdWU="

func BenchmarkTest01097(w http.ResponseWriter, r *http.Request) {
	decoded, err := base64.StdEncoding.DecodeString(benchmarkTest01097EncodedKey)
	if err != nil {
		http.Error(w, "decode error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"key_len": fmt.Sprintf("%d", len(decoded))})
}
