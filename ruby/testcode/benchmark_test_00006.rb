require_relative 'shared'

def no_marshal_ever_handler(req)
  data = req.post('data')
  obj = JSON.parse(data)
  BenchmarkResponse.json({ result: obj })
end
