require_relative 'shared'

# vuln-code-snippet start ruby_regex_url_validate_bad
def validate_url(req)
  url = req.param('url')
  matched = url.match(%r{\Ahttps?://([a-zA-Z0-9\-]+\.)+[a-zA-Z]{2,}(/\S*)?\z}) # vuln-code-snippet vuln-line ruby_regex_url_validate_bad
  BenchmarkResponse.ok(matched ? 'valid' : 'invalid')
end
# vuln-code-snippet end ruby_regex_url_validate_bad
