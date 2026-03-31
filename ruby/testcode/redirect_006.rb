require_relative 'shared'

# vuln-code-snippet start ruby_redirect_path_only
def path_only_redirect(req)
  url = req.param('url')
  parsed = URI.parse(url) rescue nil
  if parsed && parsed.host.nil? && parsed.scheme.nil? # vuln-code-snippet safe-line ruby_redirect_path_only
    BenchmarkResponse.redirect(url)
  else
    BenchmarkResponse.bad_request('only relative paths allowed')
  end
end
# vuln-code-snippet end ruby_redirect_path_only
