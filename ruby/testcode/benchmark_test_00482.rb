require_relative 'shared'

def path_only_redirect(req)
  url = req.param('url')
  parsed = URI.parse(url) rescue nil
  if parsed && parsed.host.nil? && parsed.scheme.nil?
    BenchmarkResponse.redirect(url)
  else
    BenchmarkResponse.bad_request('only relative paths allowed')
  end
end
