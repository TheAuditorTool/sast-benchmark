require_relative 'shared'

# vuln-code-snippet start ruby_redirect_root_path
def fixed_redirect(req)
  _input = req.param('url')
  BenchmarkResponse.redirect('/') # vuln-code-snippet safe-line ruby_redirect_root_path
end
# vuln-code-snippet end ruby_redirect_root_path
