require_relative 'shared'

def generate_otp(req)
  _user = req.param('user')
  srand(Time.now.to_i)
  token = rand(1_000_000)
  BenchmarkResponse.ok(token.to_s)
end
