package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

func BenchmarkTest00573(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	filter := "(|(uid=" + username + ")(mail=" + username + "))"
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeWholeSubtree,
		ldap.NeverDerefAliases,
		0, 0, false,
		filter,
		[]string{"dn", "cn", "mail"},
		nil,
	)
	result, err := LDAPConn.Search(searchReq)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
