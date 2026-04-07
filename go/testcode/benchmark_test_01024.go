package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01024Conn *ldap.Conn

func BenchmarkTest01024(w http.ResponseWriter, r *http.Request) {
	ou := r.URL.Query().Get("ou")
	baseDN := "ou=" + ou + ",dc=example,dc=com"
	searchReq := ldap.NewSearchRequest(
		baseDN,
		ldap.ScopeSingleLevel, ldap.NeverDerefAliases, 0, 0, false,
		"(objectClass=person)", []string{"cn", "mail"}, nil,
	)
	result, err := benchmarkTest01024Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
