require_relative 'shared'

# vuln-code-snippet start ruby_regex_word_boundary_loop
def validate_sentence(req)
  text = req.param('text')
  matched = text.match(/(\w+\s?)+$/) # vuln-code-snippet vuln-line ruby_regex_word_boundary_loop
  BenchmarkResponse.ok(matched ? 'valid' : 'invalid')
end
# vuln-code-snippet end ruby_regex_word_boundary_loop
