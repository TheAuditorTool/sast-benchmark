require_relative 'shared'

def activate_extension(req)
  ext_name = req.param('extension')
  require_relative("extensions/#{ext_name}")
  BenchmarkResponse.ok("extension activated")
end
