package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00402(w http.ResponseWriter, r *http.Request) {
	pattern := r.URL.Query().Get("pattern")
	out, err := exec.Command("sh", "-c", "grep "+pattern+" /var/log/app.log | wc -l").Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"count": string(out)})
}
