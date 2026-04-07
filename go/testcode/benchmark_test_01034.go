package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01034Conn *ldap.Conn

func BenchmarkTest01034(w http.ResponseWriter, r *http.Request) {
	groupDN := r.URL.Query().Get("group_dn")
	filter := "(memberOf=" + groupDN + ")"
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
		filter, []string{"cn"}, nil,
	)
	result, err := benchmarkTest01034Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
