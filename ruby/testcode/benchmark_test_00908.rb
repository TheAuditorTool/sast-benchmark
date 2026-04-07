require_relative 'shared'

def fetch_resource(req)
  raw = req.param('path')
  sanitized = raw.gsub('..', '').gsub(/\A\/+/, '')
  content = File.read("/resources/#{sanitized}")
  BenchmarkResponse.ok(content)
end
