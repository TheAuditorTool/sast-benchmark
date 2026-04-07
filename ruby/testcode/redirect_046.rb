require_relative 'shared'

# vuln-code-snippet start ruby_redirect_referer_check
def redirect_referer_check(req)
  referer = req.header('HTTP_REFERER').to_s
  dest = referer.include?('example.com') ? referer : '/'
  BenchmarkResponse.new(302, { 'Location' => dest }, '') # vuln-code-snippet safe-line ruby_redirect_referer_check
end
# vuln-code-snippet end ruby_redirect_referer_check
