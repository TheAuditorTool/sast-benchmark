require_relative 'shared'

def convert_image(req)
  filename = req.post('filename')
  exec("convert #{filename} output.png")
  BenchmarkResponse.ok("conversion started")
end
