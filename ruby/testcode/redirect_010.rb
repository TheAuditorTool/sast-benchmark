require_relative 'shared'

# vuln-code-snippet start ruby_redirect_root
def redirect_to_root(req)
  BenchmarkResponse.redirect('/') # vuln-code-snippet safe-line ruby_redirect_root
end
# vuln-code-snippet end ruby_redirect_root
