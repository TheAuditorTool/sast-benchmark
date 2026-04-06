require_relative 'shared'

# vuln-code-snippet start ruby_redirect_protocol_relative
def redirect_protocol(req)
  domain = req.param('domain')
  BenchmarkResponse.redirect("//#{domain}/callback") # vuln-code-snippet vuln-line ruby_redirect_protocol_relative
end
# vuln-code-snippet end ruby_redirect_protocol_relative
