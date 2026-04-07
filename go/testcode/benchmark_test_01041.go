package testcode

import (
	"fmt"
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01041Conn *ldap.Conn

func BenchmarkTest01041(w http.ResponseWriter, r *http.Request) {
	user := ldap.EscapeFilter(r.FormValue("username"))
	pass := ldap.EscapeFilter(r.FormValue("password"))
	filter := fmt.Sprintf("(&(uid=%s)(userPassword=%s))", user, pass)
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
		filter, []string{"dn"}, nil,
	)
	result, err := benchmarkTest01041Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"authenticated": len(result.Entries)})
}
