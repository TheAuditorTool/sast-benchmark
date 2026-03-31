require_relative 'shared'

# vuln-code-snippet start ruby_xss_json_in_script
def xss_json_in_script(req)
  username = req.param('username')
  locale = req.param('locale', default: 'en')
  user_data = "{ name: '#{username}', locale: '#{locale}' }"
  page = "<html><head><script>var appConfig = #{user_data};</script></head><body></body></html>" # vuln-code-snippet vuln-line ruby_xss_json_in_script
  BenchmarkResponse.html(page)
end
# vuln-code-snippet end ruby_xss_json_in_script
