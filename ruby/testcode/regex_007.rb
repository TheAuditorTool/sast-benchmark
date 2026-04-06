require_relative 'shared'

# vuln-code-snippet start ruby_regex_overlapping_alt
def match_overlapping(req)
  text = req.param('text')
  matched = text.match(/(a|a)+$/) # vuln-code-snippet vuln-line ruby_regex_overlapping_alt
  BenchmarkResponse.ok(matched ? 'match' : 'no match')
end
# vuln-code-snippet end ruby_regex_overlapping_alt
