require_relative 'shared'

def download_resource(req)
  resource = req.param('resource')
  content = File.read("/var/app/resources/#{resource}")
  BenchmarkResponse.ok(content)
end
