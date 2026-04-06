require_relative 'shared'

# vuln-code-snippet start ruby_redirect_concat_host
def redirect_concat(req)
  host = req.param('host')
  BenchmarkResponse.redirect("https://#{host}/callback") # vuln-code-snippet vuln-line ruby_redirect_concat_host
end
# vuln-code-snippet end ruby_redirect_concat_host
