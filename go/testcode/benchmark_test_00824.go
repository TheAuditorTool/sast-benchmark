package testcode

import (
	"encoding/base64"
	"net/http"
)

func BenchmarkTest00824(w http.ResponseWriter, r *http.Request) {
	data := r.FormValue("data")
	encrypted := base64.StdEncoding.EncodeToString([]byte(data))
	RespondJSON(w, http.StatusOK, map[string]string{"encrypted": encrypted})
}
