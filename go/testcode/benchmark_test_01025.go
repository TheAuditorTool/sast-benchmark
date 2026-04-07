package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01025Conn *ldap.Conn

func BenchmarkTest01025(w http.ResponseWriter, r *http.Request) {
	dept := r.URL.Query().Get("department")
	filter := "(department=" + dept + ")"
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
		filter, []string{"cn", "mail"}, nil,
	)
	result, err := benchmarkTest01025Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
