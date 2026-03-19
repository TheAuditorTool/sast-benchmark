package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00396(w http.ResponseWriter, r *http.Request) {
	cmd := r.URL.Query().Get("cmd")
	os.StartProcess(cmd, []string{cmd}, &os.ProcAttr{})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "started"})
}
