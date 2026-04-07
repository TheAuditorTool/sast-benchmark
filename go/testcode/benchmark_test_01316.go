package testcode

import (
	"net/http"
	"net/mail"
)

func BenchmarkTest01316(w http.ResponseWriter, r *http.Request) {
	raw := r.FormValue("email")
	addr, err := mail.ParseAddress(raw)
	if err != nil {
		http.Error(w, "invalid email address", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"email": addr.Address})
}
