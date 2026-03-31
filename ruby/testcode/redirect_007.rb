require_relative 'shared'

# vuln-code-snippet start ruby_redirect_concat_domain
def callback_redirect(req)
  domain = req.param('domain')
  BenchmarkResponse.redirect("https://" + domain + "/callback") # vuln-code-snippet vuln-line ruby_redirect_concat_domain
end
# vuln-code-snippet end ruby_redirect_concat_domain
