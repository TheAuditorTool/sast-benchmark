package testcode

import (
	"fmt"
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01032Conn *ldap.Conn

func BenchmarkTest01032(w http.ResponseWriter, r *http.Request) {
	user := r.URL.Query().Get("username")
	email := r.URL.Query().Get("email")
	filter := fmt.Sprintf("(|(uid=%s)(mail=%s))", user, email)
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
		filter, []string{"dn", "cn"}, nil,
	)
	result, err := benchmarkTest01032Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
