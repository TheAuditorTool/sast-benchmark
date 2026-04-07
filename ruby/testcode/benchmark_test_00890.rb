require_relative 'shared'

def oj_compat_deserialize_handler(req)
  data = req.post('json')
  obj = Oj.load(data, mode: :compat)
  BenchmarkResponse.json({ result: obj.to_s })
end
