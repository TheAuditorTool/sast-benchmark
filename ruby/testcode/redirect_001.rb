require_relative 'shared'

# vuln-code-snippet start ruby_redirect_open
def open_redirect(req)
  url = req.param('url')
  BenchmarkResponse.redirect(url) # vuln-code-snippet vuln-line ruby_redirect_open
end
# vuln-code-snippet end ruby_redirect_open
