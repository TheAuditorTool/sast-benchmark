require_relative 'shared'

def process_json_request(req)
  require('json')
  body = req.body_str
  data = JSON.parse(body)
  BenchmarkResponse.ok(data.to_s)
end
