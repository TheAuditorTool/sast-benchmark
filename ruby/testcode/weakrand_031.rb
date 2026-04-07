require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_counter_session
def generate_csrf_token(req)
  session_counter = 1
  csrf_token = session_counter.to_s # vuln-code-snippet vuln-line ruby_weakrand_counter_session
  BenchmarkResponse.json({ csrf_token: csrf_token })
end
# vuln-code-snippet end ruby_weakrand_counter_session
