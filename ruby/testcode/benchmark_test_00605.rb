require_relative 'shared'
require 'json'

def handler(req)
  username = req.param('username')
  locale = req.param('locale', default: 'en')
  config = JSON.generate({ name: username, locale: locale })
  page = "<html><head><script>var appConfig = #{config};</script></head><body></body></html>"
  BenchmarkResponse.html(page)
end
