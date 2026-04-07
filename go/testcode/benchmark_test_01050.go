package testcode

import (
	"fmt"
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01050Conn *ldap.Conn

func BenchmarkTest01050(w http.ResponseWriter, r *http.Request) {
	user := ldap.EscapeFilter(r.URL.Query().Get("user"))
	dept := ldap.EscapeFilter(r.URL.Query().Get("dept"))
	filter := fmt.Sprintf("(&(uid=%s)(department=%s))", user, dept)
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
		filter, []string{"cn"}, nil,
	)
	result, err := benchmarkTest01050Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
