require_relative 'shared'

def oj_rails_deserialize_handler(req)
  data = req.post('json')
  obj = Oj.load(data, mode: :rails)
  BenchmarkResponse.json({ result: obj.to_s })
end
