require_relative 'shared'

Regexp.timeout = 1.0

def handle_timeout_global_init(req)
  input = req.param('input')
  result = /\A[a-z0-9_]+\z/.match(input)
  BenchmarkResponse.json({ valid: !result.nil? })
end
