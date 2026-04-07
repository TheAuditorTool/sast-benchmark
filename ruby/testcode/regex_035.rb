require_relative 'shared'

# vuln-code-snippet start ruby_regex_union_tainted
def handle_union_tainted(req)
  raw = req.param('patterns')
  parts = raw.split(',')
  re = Regexp.union(parts) # vuln-code-snippet vuln-line ruby_regex_union_tainted
  BenchmarkResponse.json({ matched: re.match?(req.param('input')) })
end
# vuln-code-snippet end ruby_regex_union_tainted
