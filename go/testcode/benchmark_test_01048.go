package testcode

import (
	"fmt"
	"net/http"
	"strconv"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01048Conn *ldap.Conn

func BenchmarkTest01048(w http.ResponseWriter, r *http.Request) {
	idStr := r.URL.Query().Get("id")
	id, err := strconv.Atoi(idStr)
	if err != nil || id <= 0 {
		http.Error(w, "invalid id", http.StatusBadRequest)
		return
	}
	filter := fmt.Sprintf("(uidNumber=%d)", id)
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
		filter, []string{"cn", "mail"}, nil,
	)
	result, err := benchmarkTest01048Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
