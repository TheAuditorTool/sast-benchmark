require_relative 'shared'

# vuln-code-snippet start ruby_regex_split_tainted
def handle_split_tainted(req)
  input = req.param('input')
  delim = req.param('delim')
  parts = input.split(Regexp.new(delim)) # vuln-code-snippet vuln-line ruby_regex_split_tainted
  BenchmarkResponse.json({ parts: parts })
end
# vuln-code-snippet end ruby_regex_split_tainted
