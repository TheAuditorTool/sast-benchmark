require_relative 'shared'

# vuln-code-snippet start ruby_csrf_jsonp_leak
def jsonp_api(req)
  callback = req.param('callback')
  csrf_token = req.cookie('csrf_token')
  # JSONP embeds token in JavaScript response — cross-origin <script> tag reads it
  response_body = "#{callback}({\"csrf_token\":\"#{csrf_token}\",\"user\":\"admin\"})"  # vuln-code-snippet vuln-line ruby_csrf_jsonp_leak
  BenchmarkResponse.json({ result: response_body })
end
# vuln-code-snippet end ruby_csrf_jsonp_leak
