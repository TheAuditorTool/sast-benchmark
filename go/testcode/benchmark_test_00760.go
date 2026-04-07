package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
	"strings"
)

func BenchmarkTest00760(w http.ResponseWriter, r *http.Request) {
	perm := rand.Perm(10)
	parts := make([]string, len(perm))
	for i, v := range perm {
		parts[i] = fmt.Sprintf("%d", v)
	}
	token := strings.Join(parts, "")
	RespondJSON(w, http.StatusOK, map[string]string{"auth_token": token})
}
