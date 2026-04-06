require_relative 'shared'

# vuln-code-snippet start ruby_regex_length_then_match
def validate_with_length(req)
  text = req.param('text')
  return BenchmarkResponse.bad_request('too long') if text.length > 100
  matched = text.match(/(a+)+$/) # vuln-code-snippet safe-line ruby_regex_length_then_match
  BenchmarkResponse.ok(matched ? 'match' : 'no match')
end
# vuln-code-snippet end ruby_regex_length_then_match
