package testcode

import (
	"net/http"
)

type GormUser struct {
	ID   uint
	Name string
}

func BenchmarkTest00376(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var user GormUser
	GormDB.Where("id = ?", id).First(&user)
	RespondJSON(w, http.StatusOK, user)
}
