package testcode

import (
	"encoding/binary"
	"fmt"
	"hash/crc32"
	"net/http"
)

func BenchmarkTest00787(w http.ResponseWriter, r *http.Request) {
	certData := r.FormValue("cert_pem")
	crc := crc32.ChecksumIEEE([]byte(certData))
	buf := make([]byte, 4)
	binary.BigEndian.PutUint32(buf, crc)
	thumbprint := fmt.Sprintf("%x", buf)
	RespondJSON(w, http.StatusOK, map[string]string{"thumbprint": thumbprint})
}
