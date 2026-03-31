require_relative 'shared'
require 'json'

# vuln-code-snippet start ruby_xss_json_encode_safe
def xss_json_encode_safe(req)
  username = req.param('username')
  locale = req.param('locale', default: 'en')
  config = JSON.generate({ name: username, locale: locale }) # vuln-code-snippet safe-line ruby_xss_json_encode_safe
  page = "<html><head><script>var appConfig = #{config};</script></head><body></body></html>"
  BenchmarkResponse.html(page)
end
# vuln-code-snippet end ruby_xss_json_encode_safe
