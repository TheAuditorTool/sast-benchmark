package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest01309(w http.ResponseWriter, r *http.Request) {
	arr := []string{"alpha", "beta", "gamma"}
	i, err := strconv.Atoi(r.URL.Query().Get("idx"))
	if err != nil || i < 0 || i >= len(arr) {
		http.Error(w, "index out of range", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"value": arr[i]})
}
