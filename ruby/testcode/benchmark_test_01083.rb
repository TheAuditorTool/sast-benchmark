require_relative 'shared'

def handler(req)
  username = req.param('username')
  locale = req.param('locale', default: 'en')
  user_data = "{ name: '#{username}', locale: '#{locale}' }"
  page = "<html><head><script>var appConfig = #{user_data};</script></head><body></body></html>"
  BenchmarkResponse.html(page)
end
