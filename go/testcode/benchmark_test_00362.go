package testcode

import (
	"fmt"
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

func BenchmarkTest00362(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("search")
	filter := fmt.Sprintf("(|(uid=%s)(mail=%s))", input, input)
	searchReq := ldap.NewSearchRequest("dc=example,dc=com", ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false, filter, []string{"dn", "cn", "mail"}, nil)
	result, err := LDAPConn.Search(searchReq)
	if err != nil {
		http.Error(w, err.Error(), 500)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
