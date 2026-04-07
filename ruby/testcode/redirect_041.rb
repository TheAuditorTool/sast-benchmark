require_relative 'shared'

# vuln-code-snippet start ruby_redirect_back_safe
def redirect_back_safe(req)
  referer = req.header('HTTP_REFERER').to_s
  dest = referer.start_with?('/') ? referer : '/'
  BenchmarkResponse.new(302, { 'Location' => dest }, '') # vuln-code-snippet safe-line ruby_redirect_back_safe
end
# vuln-code-snippet end ruby_redirect_back_safe
