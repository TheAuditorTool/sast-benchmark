package testcode

import (
	"net/http"
	"strconv"
	"time"
)

func BenchmarkTest01302(w http.ResponseWriter, r *http.Request) {
	ms, _ := strconv.Atoi(r.URL.Query().Get("timeout"))
	time.Sleep(time.Duration(ms) * time.Millisecond)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "done"})
}
