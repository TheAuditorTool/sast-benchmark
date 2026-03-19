package testcode

import (
	"net/http"

	"github.com/google/uuid"
)

func BenchmarkTest00094(w http.ResponseWriter, r *http.Request) {
	_ = r.URL.Query().Get("name")
	filename := uuid.New().String() + ".dat"
	http.ServeFile(w, r, "./uploads/"+filename)
}
