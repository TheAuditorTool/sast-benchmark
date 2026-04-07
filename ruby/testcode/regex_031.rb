require_relative 'shared'

# vuln-code-snippet start ruby_regex_url_validate_redos
def handle_url_validate_redos(req)
  url = req.param('url')
  result = /^(https?:\/\/)([\da-z\.-]+)\.([a-z\.]{2,6})([\/\w \.-]*)*\/?$/.match(url) # vuln-code-snippet vuln-line ruby_regex_url_validate_redos
  BenchmarkResponse.json({ valid: !result.nil? })
end
# vuln-code-snippet end ruby_regex_url_validate_redos
