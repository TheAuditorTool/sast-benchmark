package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest00637(w http.ResponseWriter, r *http.Request) {
	sizeStr := r.URL.Query().Get("size")
	if sizeStr == "" {
		http.Error(w, "size parameter required", http.StatusBadRequest)
		return
	}

	n, _ := strconv.Atoi(sizeStr)

	buf := make([]byte, n)

	for i := range buf {
		buf[i] = 0x41
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"size":      n,
		"allocated": len(buf),
	})
}
