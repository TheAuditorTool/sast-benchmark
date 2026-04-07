package testcode

import (
	"crypto/md5"
	"fmt"
	"io"
	"net/http"
)

func BenchmarkTest00783(w http.ResponseWriter, r *http.Request) {
	h := md5.New()
	io.Copy(h, r.Body)
	checksum := fmt.Sprintf("%x", h.Sum(nil))
	RespondJSON(w, http.StatusOK, map[string]string{"integrity": checksum})
}
