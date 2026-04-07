package testcode

import (
	"crypto/tls"
	"net/http"
)

func BenchmarkTest00904(w http.ResponseWriter, r *http.Request) {
	host := r.URL.Query().Get("host")
	conn, err := tls.Dial("tcp", host+":443", &tls.Config{InsecureSkipVerify: true})
	if err != nil {
		http.Error(w, "dial error", http.StatusBadGateway)
		return
	}
	defer conn.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"peer": conn.ConnectionState().ServerName})
}
