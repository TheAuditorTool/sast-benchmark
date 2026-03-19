package testcode

import (
	"context"
	"net/http"
	"os/exec"
	"time"
)

func BenchmarkTest00059(w http.ResponseWriter, r *http.Request) {
	cmd := r.URL.Query().Get("cmd")
	args := r.URL.Query()["arg"]
	ctx, cancel := context.WithTimeout(r.Context(), 10*time.Second)
	defer cancel()
	command := exec.CommandContext(ctx, cmd, args...)
	output, err := command.CombinedOutput()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
