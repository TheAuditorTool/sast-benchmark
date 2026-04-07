package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

type benchmarkTest01019UserFilter struct {
	Username string
	Email    string
	Active   bool
}

func BenchmarkTest01019(w http.ResponseWriter, r *http.Request) {
	f := benchmarkTest01019UserFilter{
		Username: r.URL.Query().Get("username"),
		Email:    r.URL.Query().Get("email"),
		Active:   true,
	}
	filter := bson.D{
		{Key: "username", Value: f.Username},
		{Key: "email", Value: f.Email},
		{Key: "active", Value: f.Active},
	}
	cursor, err := MongoCollection.Find(context.Background(), filter)
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer cursor.Close(context.Background())
	var results []bson.M
	cursor.All(context.Background(), &results)
	RespondJSON(w, http.StatusOK, results)
}
