require_relative 'shared'

# vuln-code-snippet start ruby_regex_user_construct
def handle_regex_construct(req)
  pattern = req.param('pat')
  input = req.param('input')
  result = Regexp.new(pattern).match(input) # vuln-code-snippet vuln-line ruby_regex_user_construct
  BenchmarkResponse.json({ matched: !result.nil? })
end
# vuln-code-snippet end ruby_regex_user_construct
