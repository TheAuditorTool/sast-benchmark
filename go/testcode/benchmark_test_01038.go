package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01038Conn *ldap.Conn

func BenchmarkTest01038(w http.ResponseWriter, r *http.Request) {
	username := ldap.EscapeFilter(r.URL.Query().Get("username"))
	filter := "(uid=" + username + ")"
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
		filter, []string{"dn", "cn"}, nil,
	)
	result, err := benchmarkTest01038Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
