require_relative 'shared'

# vuln-code-snippet start ruby_redirect_internal_check
def safe_redirect(req)
  url = req.param('url')
  if url.start_with?('/') && !url.start_with?('//') # vuln-code-snippet safe-line ruby_redirect_internal_check
    BenchmarkResponse.redirect(url)
  else
    BenchmarkResponse.bad_request('invalid redirect target')
  end
end
# vuln-code-snippet end ruby_redirect_internal_check
