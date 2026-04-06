require_relative 'shared'

# vuln-code-snippet start ruby_redirect_param_return
def login_redirect(req)
  return_url = req.param('return_url')
  BenchmarkResponse.redirect(return_url) # vuln-code-snippet vuln-line ruby_redirect_param_return
end
# vuln-code-snippet end ruby_redirect_param_return
