require_relative 'shared'

def sensitive_action(req)
  BenchmarkResponse.ok("action performed")
end
