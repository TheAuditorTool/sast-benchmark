package testcode

import (
	"html/template"
	"net/http"
)

var benchmarkTest00704Template = template.Must(template.New("cb").Parse(`<script>var cb = {{.Callback}};</script>`))

func BenchmarkTest00704(w http.ResponseWriter, r *http.Request) {
	callback := r.URL.Query().Get("callback")
	data := struct {
		Callback template.JS
	}{
		Callback: template.JS(callback),
	}
	w.Header().Set("Content-Type", "text/html")
	benchmarkTest00704Template.Execute(w, data)
}
