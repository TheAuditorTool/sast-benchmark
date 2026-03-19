package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00397(w http.ResponseWriter, r *http.Request) {
	proc, err := os.StartProcess("/usr/bin/date", []string{"date"}, &os.ProcAttr{
		Files: []*os.File{os.Stdin, os.Stdout, os.Stderr},
	})
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	proc.Wait()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "done"})
}
