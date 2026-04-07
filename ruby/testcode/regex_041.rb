require_relative 'shared'

# vuln-code-snippet start ruby_regex_re2_gem
def handle_re2_gem(req)
  pat = req.param('pat')
  input = req.param('input')
  result = RE2::Regexp.new(pat).match(input) # vuln-code-snippet safe-line ruby_regex_re2_gem
  BenchmarkResponse.json({ matched: !result.nil? })
end
# vuln-code-snippet end ruby_regex_re2_gem
