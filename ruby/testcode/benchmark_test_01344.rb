require_relative 'shared'

def set_locale_header(req)
  locale = req.param('locale').to_s.gsub(/[\r\n]/, '')
  resp = BenchmarkResponse.ok('ok')
  resp.add_header('Content-Language', locale)
  resp
end
