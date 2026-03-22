package testcode

import (
	"bytes"
	"net/http"
	"text/template"
)

var benchmarkTest00493Template = template.Must(template.New("greeting").Parse("Hello {{.Name}}"))

func BenchmarkTest00493(w http.ResponseWriter, r *http.Request) {
	name := r.FormValue("name")
	data := map[string]string{
		"Name": name,
	}

	var buf bytes.Buffer
	if err := benchmarkTest00493Template.Execute(&buf, data); err != nil {
		http.Error(w, "render error", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	w.Write(buf.Bytes())
}
