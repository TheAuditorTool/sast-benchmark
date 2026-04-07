package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01027Conn *ldap.Conn

func BenchmarkTest01027(w http.ResponseWriter, r *http.Request) {
	userDN := r.FormValue("dn")
	cn := r.FormValue("cn")
	addReq := ldap.NewAddRequest(userDN, nil)
	addReq.Attribute("objectClass", []string{"inetOrgPerson"})
	addReq.Attribute("cn", []string{cn})
	if err := benchmarkTest01027Conn.Add(addReq); err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "added"})
}
