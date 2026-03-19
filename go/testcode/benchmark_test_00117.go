package testcode

import (
	"net/http"
	tmpl "text/template"
)

func BenchmarkTest00117(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("name")
	t, err := tmpl.New("page").Parse(`<html><body><h1>Hello, {{.Name}}</h1></body></html>`)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	data := struct {
		Name string
	}{
		Name: userInput,
	}
	w.Header().Set("Content-Type", "text/html")
	t.Execute(w, data)
}
