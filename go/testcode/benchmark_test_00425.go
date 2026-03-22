package testcode

import (
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest00425(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	filter := bson.M{"$where": "this.username == '" + username + "'"}
	cursor, err := MongoCollection.Find(r.Context(), filter)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer cursor.Close(r.Context())
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
