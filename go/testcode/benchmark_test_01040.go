package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01040Conn *ldap.Conn

var benchmarkTest01040OUs = map[string]string{
	"eng":     "ou=Engineering,dc=example,dc=com",
	"sales":   "ou=Sales,dc=example,dc=com",
	"support": "ou=Support,dc=example,dc=com",
}

func BenchmarkTest01040(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("dept")
	baseDN, ok := benchmarkTest01040OUs[key]
	if !ok {
		http.Error(w, "invalid department", http.StatusBadRequest)
		return
	}
	searchReq := ldap.NewSearchRequest(
		baseDN,
		ldap.ScopeSingleLevel, ldap.NeverDerefAliases, 0, 0, false,
		"(objectClass=person)", []string{"cn", "mail"}, nil,
	)
	result, err := benchmarkTest01040Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
