package testcode

import (
	"fmt"
	"log"
	"net/http"
)

func BenchmarkTest00990(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	email := r.FormValue("email")
	msg := fmt.Sprintf("register user=%q email=%q", username, email)
	log.Println(msg)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
