require_relative 'shared'

Regexp.timeout = 1.0 # Ruby 3.2+

# vuln-code-snippet start ruby_regex_timeout_global_init
def handle_timeout_global_init(req)
  input = req.param('input')
  result = /\A[a-z0-9_]+\z/.match(input) # vuln-code-snippet safe-line ruby_regex_timeout_global_init
  BenchmarkResponse.json({ valid: !result.nil? })
end
# vuln-code-snippet end ruby_regex_timeout_global_init
