package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01036Conn *ldap.Conn

func BenchmarkTest01036(w http.ResponseWriter, r *http.Request) {
	role := r.URL.Query().Get("role")
	username := r.URL.Query().Get("username")
	filter := "(&(uid=" + username + ")(title=" + role + "))"
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
		filter, []string{"dn"}, nil,
	)
	result, err := benchmarkTest01036Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
