require_relative 'shared'

def generate_token(req)
  srand(Process.pid)
  token = rand(2**128)
  BenchmarkResponse.json({ token: token })
end
