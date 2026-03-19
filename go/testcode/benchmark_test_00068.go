package testcode

import (
	"net/http"
	"os"
	"os/exec"
)

func BenchmarkTest00068(w http.ResponseWriter, r *http.Request) {
	pathDir := r.URL.Query().Get("path_dir")
	os.Setenv("LOOKUP_PATH", pathDir)
	cmd := exec.Command("custom_tool", "--process")
	cmd.Env = append(os.Environ(), "LOOKUP_PATH="+pathDir)
	output, _ := cmd.CombinedOutput()
	w.Write(output)
}
