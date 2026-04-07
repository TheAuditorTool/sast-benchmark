require_relative 'shared'

# vuln-code-snippet start ruby_regex_string_match_tainted
def handle_string_match_tainted(req)
  input = req.param('input')
  result = input.match(/^(a+)+$/) # vuln-code-snippet vuln-line ruby_regex_string_match_tainted
  BenchmarkResponse.json({ matched: !result.nil? })
end
# vuln-code-snippet end ruby_regex_string_match_tainted
