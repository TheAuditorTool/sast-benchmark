package testcode

import (
	"context"
	"net/http"
	"strconv"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
)

func BenchmarkTest01021(w http.ResponseWriter, r *http.Request) {
	minPriceStr := r.URL.Query().Get("min_price")
	minPrice, _ := strconv.ParseFloat(minPriceStr, 64)
	pipeline := mongo.Pipeline{
		bson.D{{Key: "$match", Value: bson.M{"active": true, "price": bson.M{"$gte": minPrice}}}},
		bson.D{{Key: "$sort", Value: bson.D{{Key: "price", Value: 1}}}},
		bson.D{{Key: "$limit", Value: 20}},
	}
	cursor, err := MongoCollection.Aggregate(context.Background(), pipeline)
	if err != nil {
		http.Error(w, "aggregate error", http.StatusInternalServerError)
		return
	}
	defer cursor.Close(context.Background())
	var results []bson.M
	cursor.All(context.Background(), &results)
	RespondJSON(w, http.StatusOK, results)
}
