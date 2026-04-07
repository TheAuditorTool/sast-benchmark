require_relative 'shared'

def oj_wab_deserialize_handler(req)
  data = req.post('data')
  obj = Oj.wab_load(data)
  BenchmarkResponse.json({ result: obj.to_s })
end
