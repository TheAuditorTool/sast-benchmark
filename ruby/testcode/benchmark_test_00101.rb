require_relative 'shared'
require 'json'

def parse_input(req)
  input = req.body_str
  data = JSON.parse(input)
  BenchmarkResponse.json(data)
end
