package testcode

import (
	"fmt"
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

func BenchmarkTest00360(w http.ResponseWriter, r *http.Request) {
	user := r.FormValue("user")
	pass := r.FormValue("pass")
	filter := fmt.Sprintf("(&(uid=%s)(userPassword=%s))", user, pass)
	searchReq := ldap.NewSearchRequest("dc=example,dc=com", ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false, filter, []string{"dn"}, nil)
	result, err := LDAPConn.Search(searchReq)
	if err != nil {
		http.Error(w, err.Error(), 500)
		return
	}
	authenticated := len(result.Entries) > 0
	RespondJSON(w, http.StatusOK, map[string]bool{"authenticated": authenticated})
}
