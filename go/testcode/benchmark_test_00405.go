package testcode

import (
	"bytes"
	"net/http"
	"os/exec"
	"text/template"
)

func BenchmarkTest00405(w http.ResponseWriter, r *http.Request) {
	tmpl, err := template.New("cmd").Parse("date +%Y-%m-%d")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	var buf bytes.Buffer
	tmpl.Execute(&buf, nil)
	out, err := exec.Command("sh", "-c", buf.String()).Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"date": string(out)})
}
