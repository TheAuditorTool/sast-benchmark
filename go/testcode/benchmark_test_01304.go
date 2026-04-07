package testcode

import (
	"math"
	"net/http"
	"strconv"
	"time"
)

func BenchmarkTest01304(w http.ResponseWriter, r *http.Request) {
	userMs, _ := strconv.ParseInt(r.URL.Query().Get("ms"), 10, 64)
	if userMs == math.MaxInt64 {
		userMs = math.MaxInt64
	}
	d := time.Duration(userMs) * time.Millisecond
	time.Sleep(d)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "done"})
}
