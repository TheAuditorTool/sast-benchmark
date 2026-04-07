require_relative 'shared'

def redirect_to_url_sanitized(req)
  input = req.param('url')
  safe_value = input.gsub(/[\r\n]/, '')
  BenchmarkResponse.new(302, '', { 'Location' => safe_value })
end
