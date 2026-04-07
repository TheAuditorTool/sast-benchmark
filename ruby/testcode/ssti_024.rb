require_relative 'shared'

# vuln-code-snippet start ruby_ssti_string_format_pct
def handler(req)
  output = req.param('fmt') % { name: 'world', value: 42 } # vuln-code-snippet vuln-line ruby_ssti_string_format_pct
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_string_format_pct
