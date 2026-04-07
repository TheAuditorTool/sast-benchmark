require_relative 'shared'

def psych_deserialize_handler(req)
  data = req.post('data')
  obj = Psych.load(data)
  BenchmarkResponse.json({ result: obj.to_s })
end
