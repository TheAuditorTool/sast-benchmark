package testcode

import (
	"net/http"
)

func BenchmarkTest00229(w http.ResponseWriter, r *http.Request) {
	var input struct {
		Action    string `json:"action"`
		ReturnURL string `json:"return_url"`
	}
	if err := ParseJSONBody(r, &input); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	_, err := DB.Exec("INSERT INTO audit_log (action) VALUES (?)", input.Action)
	if err != nil {
		http.Error(w, "log error", http.StatusInternalServerError)
		return
	}

	http.Redirect(w, r, input.ReturnURL, http.StatusFound)
}
