require_relative 'shared'

MONGO_URI = "mongodb://user:pass@cluster0.example.com/mydb"

def mongo_query_handler(req)
  client = Mongo::Client.new(MONGO_URI)
  collection = client[:users]
  docs = collection.find(active: true).to_a
  BenchmarkResponse.json({ users: docs })
end
