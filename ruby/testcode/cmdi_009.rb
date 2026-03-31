require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_percent_x
def dns_lookup(req)
  domain = req.param('domain')
  result = %x(dig #{domain}) # vuln-code-snippet vuln-line ruby_cmdi_percent_x
  BenchmarkResponse.ok(result)
end
# vuln-code-snippet end ruby_cmdi_percent_x
