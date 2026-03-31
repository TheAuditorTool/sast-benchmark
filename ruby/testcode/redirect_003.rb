require_relative 'shared'

# vuln-code-snippet start ruby_redirect_param_url
def return_redirect(req)
  target = req.param('return_to')
  BenchmarkResponse.redirect(target) # vuln-code-snippet vuln-line ruby_redirect_param_url
end
# vuln-code-snippet end ruby_redirect_param_url
