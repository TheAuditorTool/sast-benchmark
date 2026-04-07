require_relative 'shared'

MONGO_URI = "mongodb://user:pass@cluster0.example.com/mydb"

# vuln-code-snippet start ruby_hardcoded_mongo_uri
def mongo_query_handler(req)
  client = Mongo::Client.new(MONGO_URI)  # vuln-code-snippet vuln-line ruby_hardcoded_mongo_uri
  collection = client[:users]
  docs = collection.find(active: true).to_a
  BenchmarkResponse.json({ users: docs })
end
# vuln-code-snippet end ruby_hardcoded_mongo_uri
