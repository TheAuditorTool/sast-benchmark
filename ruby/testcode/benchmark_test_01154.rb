require_relative 'shared'

def generate_session_token(req)
  _user = req.param('user')
  token = rand(1_000_000).to_s
  BenchmarkResponse.ok(token)
end
