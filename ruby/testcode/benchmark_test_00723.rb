require_relative 'shared'

def json_parse_strict_handler(req)
  data = req.post('data')
  obj = JSON.parse(data, symbolize_names: false)
  BenchmarkResponse.json({ result: obj })
end
