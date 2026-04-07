require_relative 'shared'

# vuln-code-snippet start ruby_regex_compile_loop
def handle_compile_loop(req)
  patterns = req.param('patterns').split(',')
  text = req.param('text')
  results = patterns.map { |p| Regexp.new(p).match(text) } # vuln-code-snippet vuln-line ruby_regex_compile_loop
  BenchmarkResponse.json({ count: results.compact.length })
end
# vuln-code-snippet end ruby_regex_compile_loop
