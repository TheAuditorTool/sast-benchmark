package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var LDAPConn *ldap.Conn

func BenchmarkTest00359(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	filter := "(uid=" + username + ")"
	searchReq := ldap.NewSearchRequest("dc=example,dc=com", ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false, filter, []string{"dn", "cn"}, nil)
	result, err := LDAPConn.Search(searchReq)
	if err != nil {
		http.Error(w, err.Error(), 500)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
