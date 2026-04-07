require_relative 'shared'

# vuln-code-snippet start ruby_regex_atomic_group_match
def handle_atomic_group_match(req)
  input = req.param('input')
  result = /(?>ab|a)bc/.match(input) # vuln-code-snippet safe-line ruby_regex_atomic_group_match
  BenchmarkResponse.json({ matched: !result.nil? })
end
# vuln-code-snippet end ruby_regex_atomic_group_match
