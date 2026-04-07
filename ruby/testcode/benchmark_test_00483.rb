require_relative 'shared'

def send_file_basename(req)
  fname = File.basename(req.param('name'))
  data = File.read(File.join('/var/uploads', fname))
  BenchmarkResponse.new(200, { 'Content-Type' => 'application/octet-stream' }, [data])
end
