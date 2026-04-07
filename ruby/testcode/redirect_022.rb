require_relative 'shared'

# vuln-code-snippet start ruby_redirect_meta_refresh
def redirect_meta_refresh(req)
  html = "<meta http-equiv='refresh' content='0;url=#{req.param('url')}'>" # vuln-code-snippet vuln-line ruby_redirect_meta_refresh
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ruby_redirect_meta_refresh
