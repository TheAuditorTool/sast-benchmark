require_relative 'shared'

# vuln-code-snippet start ruby_regex_possessive_anchor
def handle_possessive_anchor(req)
  input = req.param('input')
  result = /\A\w++\z/.match(input) # Ruby 3.2+ # vuln-code-snippet safe-line ruby_regex_possessive_anchor
  BenchmarkResponse.json({ valid: !result.nil? })
end
# vuln-code-snippet end ruby_regex_possessive_anchor
