package testcode

import (
	"net/http"
	"os/exec"
	"text/template"
)

type benchmarkTest00592Task struct {
	Title    string
	Priority int
}

func (t benchmarkTest00592Task) Run(cmd string) string {
	out, _ := exec.Command("sh", "-c", cmd).Output()
	return string(out)
}

type benchmarkTest00592Body struct {
	TemplateStr string `json:"template"`
	Title       string `json:"title"`
	Priority    int    `json:"priority"`
}

func BenchmarkTest00592(w http.ResponseWriter, r *http.Request) {
	var body benchmarkTest00592Body
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	tmpl, err := template.New("task").Parse(body.TemplateStr)
	if err != nil {
		http.Error(w, "template parse error", http.StatusBadRequest)
		return
	}

	task := benchmarkTest00592Task{
		Title:    body.Title,
		Priority: body.Priority,
	}

	w.Header().Set("Content-Type", "text/plain")
	if err := tmpl.Execute(w, task); err != nil {
		http.Error(w, "execution error", http.StatusInternalServerError)
	}
}
