require_relative 'shared'

def psych_safe_empty_permitted_handler(req)
  data = req.post('data')
  obj = Psych.safe_load(data, permitted_classes: [])
  BenchmarkResponse.json({ result: obj.to_s })
end
