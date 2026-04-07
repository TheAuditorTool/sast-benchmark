require_relative 'shared'

def generate_csrf_token(req)
  _session = req.cookie('session')
  token = Random.rand(999999)
  BenchmarkResponse.ok(token.to_s)
end
