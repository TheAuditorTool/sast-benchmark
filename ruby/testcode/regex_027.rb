require_relative 'shared'

# vuln-code-snippet start ruby_regex_alternation_loop
def handle_alternation_loop(req)
  input = req.param('input')
  result = /(red|blue|green)+/.match(input) # vuln-code-snippet vuln-line ruby_regex_alternation_loop
  BenchmarkResponse.json({ matched: !result.nil? })
end
# vuln-code-snippet end ruby_regex_alternation_loop
