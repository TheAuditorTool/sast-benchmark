require_relative 'shared'

# vuln-code-snippet start ruby_regex_backtrack
def parse_sentence(req)
  input = req.param('input')
  matched = input =~ /(\w+\s+)*end/  # vuln-code-snippet vuln-line ruby_regex_backtrack
  BenchmarkResponse.ok(matched ? 'found end' : 'no end')
end
# vuln-code-snippet end ruby_regex_backtrack
