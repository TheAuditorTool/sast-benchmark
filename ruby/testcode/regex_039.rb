require_relative 'shared'

# vuln-code-snippet start ruby_regex_length_limit_input
def handle_length_limit_input(req)
  input = req.param('data')
  raise ArgumentError, 'too long' if input.length > 200
  result = /^[a-z]+$/.match(input) # vuln-code-snippet safe-line ruby_regex_length_limit_input
  BenchmarkResponse.json({ valid: !result.nil? })
end
# vuln-code-snippet end ruby_regex_length_limit_input
