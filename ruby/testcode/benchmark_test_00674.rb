require_relative 'shared'

def json_create_id_handler(req)
  data = req.post('json')
  obj = JSON.load(data)
  BenchmarkResponse.json({ result: obj.to_s })
end
