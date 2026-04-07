package testcode

import (
	"net/http"
	"strconv"
	"time"
)

func BenchmarkTest01321(w http.ResponseWriter, r *http.Request) {
	ms, _ := strconv.Atoi(r.URL.Query().Get("timeout"))
	if ms < 1 || ms > 5000 {
		ms = 1000
	}
	time.Sleep(time.Duration(ms) * time.Millisecond)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "done"})
}
