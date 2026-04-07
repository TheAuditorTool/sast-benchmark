package testcode

import (
	"fmt"
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01043Conn *ldap.Conn

func BenchmarkTest01043(w http.ResponseWriter, r *http.Request) {
	username := ldap.EscapeFilter(r.URL.Query().Get("username"))
	filter := fmt.Sprintf("(&(objectClass=person)(uid=%s))", username)
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
		filter, []string{"dn", "cn", "mail"}, nil,
	)
	result, err := benchmarkTest01043Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
