require_relative 'shared'

# vuln-code-snippet start ruby_regex_possessive_quant
def match_possessive(req)
  text = req.param('text')
  matched = text.match(/\A(a++)b\z/) # vuln-code-snippet safe-line ruby_regex_possessive_quant
  BenchmarkResponse.ok(matched ? 'match' : 'no match')
end
# vuln-code-snippet end ruby_regex_possessive_quant
