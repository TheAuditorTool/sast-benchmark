package testcode

import (
	"bytes"
	"net/http"
	"os/exec"
	"text/template"
)

func BenchmarkTest00404(w http.ResponseWriter, r *http.Request) {
	host := r.URL.Query().Get("host")
	tmpl, err := template.New("cmd").Parse("ping -c 1 {{.Host}}")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	var buf bytes.Buffer
	tmpl.Execute(&buf, map[string]string{"Host": host})
	out, err := exec.Command("sh", "-c", buf.String()).Output()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"result": string(out)})
}
