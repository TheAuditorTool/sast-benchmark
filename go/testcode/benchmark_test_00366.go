package testcode

import (
	"fmt"
	"net/http"
	"regexp"

	"github.com/go-ldap/ldap/v3"
)

var alphanumericPattern = regexp.MustCompile("^[a-zA-Z0-9]+$")

func BenchmarkTest00366(w http.ResponseWriter, r *http.Request) {
	uid := r.URL.Query().Get("uid")
	if !alphanumericPattern.MatchString(uid) {
		http.Error(w, "invalid uid", http.StatusBadRequest)
		return
	}
	filter := fmt.Sprintf("(uid=%s)", uid)
	searchReq := ldap.NewSearchRequest("dc=example,dc=com", ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false, filter, []string{"dn", "cn"}, nil)
	result, err := LDAPConn.Search(searchReq)
	if err != nil {
		http.Error(w, err.Error(), 500)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
