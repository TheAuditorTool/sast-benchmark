require_relative 'shared'

def set_link_header(req)
  url = req.param('url')
  BenchmarkResponse.new(200, 'ok', { 'Link' => "<#{url}>; rel=\"canonical\"" })
end
