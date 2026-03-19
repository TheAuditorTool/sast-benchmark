package testcode

import (
	"net/http"
	"os/exec"
)

func BenchmarkTest00262(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("flag")
	arg := param
	if 2*2 == 4 {
		arg = "-la"
	}
	output, err := exec.Command("ls", arg).Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
