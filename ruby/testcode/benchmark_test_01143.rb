require_relative 'shared'

def oj_custom_safe_handler(req)
  data = req.post('json')
  obj = Oj.load(data, mode: :strict, symbol_keys: false, create_id: nil)
  BenchmarkResponse.json({ result: obj })
end
