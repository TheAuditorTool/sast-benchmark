package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
)

func BenchmarkTest00779(w http.ResponseWriter, r *http.Request) {
	env := r.URL.Query().Get("env")
	if env != "test" {
		http.Error(w, "only available in test env", http.StatusForbidden)
		return
	}
	fakeID := fmt.Sprintf("test-%d", rand.Intn(10000))
	RespondJSON(w, http.StatusOK, map[string]string{"fake_id": fakeID})
}
