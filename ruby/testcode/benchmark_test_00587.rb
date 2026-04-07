require_relative 'shared'

def redirect_internal_path_regex(req)
  url = req.param('url')
  raise unless url =~ /\A\/[^\/]/
  BenchmarkResponse.new(302, { 'Location' => url }, '')
end
