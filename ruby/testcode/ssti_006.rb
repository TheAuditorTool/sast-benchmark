require_relative 'shared'

GREETING_TEMPLATE = 'Welcome, {{name}}!'.freeze

# vuln-code-snippet start ruby_ssti_safe_substitution
def ssti_safe_substitution(req)
  name = escape_html(req.param('name'))
  output = GREETING_TEMPLATE.gsub('{{name}}', name) # vuln-code-snippet safe-line ruby_ssti_safe_substitution
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_safe_substitution
