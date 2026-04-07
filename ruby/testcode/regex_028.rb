require_relative 'shared'

# vuln-code-snippet start ruby_regex_lookahead_loop
def handle_lookahead_loop(req)
  input = req.param('input')
  result = /(?=.*a)(?=.*b)+/.match(input) # vuln-code-snippet vuln-line ruby_regex_lookahead_loop
  BenchmarkResponse.json({ matched: !result.nil? })
end
# vuln-code-snippet end ruby_regex_lookahead_loop
