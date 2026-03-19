package testcode

import (
	"html/template"
	"net/http"
)

func BenchmarkTest00114(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("style")
	t, err := template.New("page").Parse(`<html><head><style>{{.Style}}</style></head><body>Hello</body></html>`)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	data := struct {
		Style template.CSS
	}{
		Style: template.CSS(userInput),
	}
	w.Header().Set("Content-Type", "text/html")
	t.Execute(w, data)
}
