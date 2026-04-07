require_relative 'shared'
require 'json'

def parse_json_alternative(req)
  data = JSON.parse(req.body_str)
  BenchmarkResponse.json(data)
end
