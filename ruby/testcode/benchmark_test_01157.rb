require_relative 'shared'

def redirect_safe(req)
  url = req.param('url')
  safe_url = url.delete("\r\n")
  BenchmarkResponse.new(302, '', { 'Location' => safe_url })
end
