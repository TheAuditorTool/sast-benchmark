require_relative 'shared'

def oj_strict_mode_handler(req)
  data = req.post('json')
  obj = Oj.load(data, mode: :strict)
  BenchmarkResponse.json({ result: obj })
end
