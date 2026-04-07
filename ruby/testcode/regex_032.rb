require_relative 'shared'

# vuln-code-snippet start ruby_regex_user_flags
def handle_user_flags(req)
  pat = req.param('pat')
  input = req.param('input')
  result = Regexp.new(pat, Regexp::IGNORECASE | Regexp::EXTENDED).match(input) # vuln-code-snippet vuln-line ruby_regex_user_flags
  BenchmarkResponse.json({ matched: !result.nil? })
end
# vuln-code-snippet end ruby_regex_user_flags
