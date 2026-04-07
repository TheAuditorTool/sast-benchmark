require_relative 'shared'
require 'uri'

def redirect_path_parsed(req)
  url = req.param('url')
  parsed = URI.parse(url) rescue nil
  safe_path = parsed&.path || '/'
  BenchmarkResponse.redirect(safe_path)
end
