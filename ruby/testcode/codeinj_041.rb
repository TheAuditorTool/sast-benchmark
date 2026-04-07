require_relative 'shared'

begin
  require 'prism'
rescue LoadError
end

# vuln-code-snippet start ruby_codeinj_prism_parse
def parse_ruby_source(req)
  source = req.param('code')
  parse_result = Prism.parse(source) # vuln-code-snippet safe-line ruby_codeinj_prism_parse
  BenchmarkResponse.json({ success: parse_result.success?, errors: parse_result.errors.map(&:message) })
end
# vuln-code-snippet end ruby_codeinj_prism_parse
