package testcode

import (
	"bytes"
	"net/http"
	"os/exec"
	"text/template"
)

var benchmarkTest00491FuncMap = template.FuncMap{
	"exec": func(cmd string) string {
		out, err := exec.Command("sh", "-c", cmd).CombinedOutput()
		if err != nil {
			return err.Error()
		}
		return string(out)
	},
}

func BenchmarkTest00491(w http.ResponseWriter, r *http.Request) {
	tmplStr := r.FormValue("tmpl")
	if tmplStr == "" {
		http.Error(w, "missing tmpl parameter", http.StatusBadRequest)
		return
	}

	tmpl, err := template.New("user").Funcs(benchmarkTest00491FuncMap).Parse(tmplStr)
	if err != nil {
		http.Error(w, "template parse error", http.StatusBadRequest)
		return
	}

	data := map[string]string{
		"Name": r.FormValue("name"),
	}

	var buf bytes.Buffer
	if err := tmpl.Execute(&buf, data); err != nil {
		http.Error(w, "template exec error", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	w.Write(buf.Bytes())
}
