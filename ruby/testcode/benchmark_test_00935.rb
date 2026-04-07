require_relative 'shared'

def protected_endpoint_rack(req)
  authenticity_token = req.header('X-CSRF-Token') || req.post('authenticity_token')
  raise 'CSRF validation must happen in middleware' unless authenticity_token
  data = req.post('data')
  BenchmarkResponse.json({ result: data })
end
