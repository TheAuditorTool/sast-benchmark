package testcode

import (
	"net/http"
	"os"
	"syscall"
)

func BenchmarkTest00395(w http.ResponseWriter, r *http.Request) {
	cmd := r.URL.Query().Get("cmd")
	args := []string{cmd}
	env := os.Environ()
	syscall.Exec(cmd, args, env)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "executed"})
}
