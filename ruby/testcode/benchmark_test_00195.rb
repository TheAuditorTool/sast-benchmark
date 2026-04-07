require_relative 'shared'

def set_link_preload(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Link'] = "<#{req.param('url')}>; rel=preload"
  response
end
