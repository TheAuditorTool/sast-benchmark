require_relative 'shared'
require 'json'

def create_from_json(req)
  attrs = JSON.parse(req.body_str)
  BenchmarkResponse.ok("created: #{attrs}")
end
