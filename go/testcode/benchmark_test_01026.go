package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01026Conn *ldap.Conn

func BenchmarkTest01026(w http.ResponseWriter, r *http.Request) {
	dn := r.FormValue("dn")
	attrName := r.FormValue("attr")
	attrVal := r.FormValue("value")
	modReq := ldap.NewModifyRequest(dn, nil)
	modReq.Replace(attrName, []string{attrVal})
	if err := benchmarkTest01026Conn.Modify(modReq); err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "modified"})
}
