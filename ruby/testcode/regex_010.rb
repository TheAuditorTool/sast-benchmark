require_relative 'shared'

# vuln-code-snippet start ruby_regex_atomic_group
def match_atomic(req)
  text = req.param('text')
  matched = text.match(/(?>a+)b/) # vuln-code-snippet safe-line ruby_regex_atomic_group
  BenchmarkResponse.ok(matched ? 'match' : 'no match')
end
# vuln-code-snippet end ruby_regex_atomic_group
