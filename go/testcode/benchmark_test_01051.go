package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
	"github.com/gorilla/sessions"
)

var benchmarkTest01051Conn *ldap.Conn
var benchmarkTest01051Store = sessions.NewCookieStore([]byte("session-key"))

func BenchmarkTest01051(w http.ResponseWriter, r *http.Request) {
	sess, err := benchmarkTest01051Store.Get(r, "session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	username, ok := sess.Values["username"].(string)
	if !ok || username == "" {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	filter := "(uid=" + ldap.EscapeFilter(username) + ")"
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
		filter, []string{"cn", "mail"}, nil,
	)
	result, err2 := benchmarkTest01051Conn.Search(searchReq)
	if err2 != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
