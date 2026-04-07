require_relative 'shared'
require 'json'

def parse_json_instead_handler(req)
  data = req.post('data')
  obj = JSON.parse(data)
  BenchmarkResponse.json({ result: obj })
end
