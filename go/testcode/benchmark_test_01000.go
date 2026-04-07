package testcode

import (
	"context"
	"net/http"
	"sync"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest01000(w http.ResponseWriter, r *http.Request) {
	field := r.URL.Query().Get("field")
	value := r.URL.Query().Get("value")
	var results []bson.M
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		cursor, err := MongoCollection.Find(context.Background(), bson.M{field: value})
		if err == nil {
			cursor.All(context.Background(), &results)
			cursor.Close(context.Background())
		}
	}()
	wg.Wait()
	RespondJSON(w, http.StatusOK, results)
}
