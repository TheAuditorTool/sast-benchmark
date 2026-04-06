package testcode

import (
	"net/http"
	"text/template"
)

type benchmarkTest00596ReportData struct {
	Title string `json:"title"`
	Count int    `json:"count"`
}

var benchmarkTest00596Tmpl = template.Must(
	template.New("report").Parse("Report for {{.Title}} - Items: {{.Count}}"),
)

func BenchmarkTest00596(w http.ResponseWriter, r *http.Request) {
	var data benchmarkTest00596ReportData
	if err := ParseJSONBody(r, &data); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	if data.Title == "" {
		http.Error(w, "title required", http.StatusUnprocessableEntity)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	if err := benchmarkTest00596Tmpl.Execute(w, data); err != nil {
		http.Error(w, "render error", http.StatusInternalServerError)
	}
}
