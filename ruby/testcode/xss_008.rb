require_relative 'shared'

# vuln-code-snippet start ruby_xss_content_type_json
def xss_content_type_json(req)
  user_id = req.param('user_id')
  filter = req.param('filter', default: 'active')
  results = [
    { id: user_id, filter: filter, status: 'ok' }
  ]
  BenchmarkResponse.json(results) # vuln-code-snippet safe-line ruby_xss_content_type_json
end
# vuln-code-snippet end ruby_xss_content_type_json
