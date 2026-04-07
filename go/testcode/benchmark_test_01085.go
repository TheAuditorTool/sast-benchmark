package testcode

import (
	"fmt"
	"net/http"
)

const benchmarkTest01085GitHubToken = "ghp_aBcDeFgHiJkLmNoPqRsTuVwXyZ123456"

func BenchmarkTest01085(w http.ResponseWriter, r *http.Request) {
	authHeader := fmt.Sprintf("token %s", benchmarkTest01085GitHubToken)
	RespondJSON(w, http.StatusOK, map[string]string{"auth": authHeader})
}
