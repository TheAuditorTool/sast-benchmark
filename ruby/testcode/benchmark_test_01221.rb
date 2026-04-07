require_relative 'shared'

def redirect_root_fallback(req)
  url = req.param('url')
  dest = url.start_with?('/') && !url.start_with?('//') ? url : '/'
  BenchmarkResponse.new(302, { 'Location' => dest }, '')
end
