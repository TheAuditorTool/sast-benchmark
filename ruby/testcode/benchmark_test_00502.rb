require_relative 'shared'

def generate_csrf_token(req)
  session_counter = 1
  csrf_token = session_counter.to_s
  BenchmarkResponse.json({ csrf_token: csrf_token })
end
