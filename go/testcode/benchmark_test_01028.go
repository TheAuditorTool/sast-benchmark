package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01028Conn *ldap.Conn

func benchmarkTest01028BuildFilter(username string) string {
	return "(uid=" + username + ")"
}

func BenchmarkTest01028(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	filter := benchmarkTest01028BuildFilter(username)
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
		filter, []string{"dn"}, nil,
	)
	result, err := benchmarkTest01028Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
