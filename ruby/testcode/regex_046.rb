require_relative 'shared'

# vuln-code-snippet start ruby_regex_length_then_simple
def handle_length_then_simple(req)
  val = req.param('code')
  raise ArgumentError if val.length > 100
  result = val.match?(/\A\d+\z/) # vuln-code-snippet safe-line ruby_regex_length_then_simple
  BenchmarkResponse.json({ valid: result })
end
# vuln-code-snippet end ruby_regex_length_then_simple
