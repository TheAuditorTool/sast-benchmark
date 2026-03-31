require_relative 'shared'

# vuln-code-snippet start ruby_xss_text_only
def xss_text_only(req)
  message = req.param('message')
  lines = message.split("\n").first(10).join("\n")
  BenchmarkResponse.ok(lines) # vuln-code-snippet safe-line ruby_xss_text_only
end
# vuln-code-snippet end ruby_xss_text_only
