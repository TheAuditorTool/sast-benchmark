package testcode

import (
	"fmt"
	"net/http"
)

const benchmarkTest01089APIKey = "abc123secret-key-value"

func BenchmarkTest01089(w http.ResponseWriter, r *http.Request) {
	header := fmt.Sprintf("Bearer %s", benchmarkTest01089APIKey)
	RespondJSON(w, http.StatusOK, map[string]string{"auth_header": header})
}
