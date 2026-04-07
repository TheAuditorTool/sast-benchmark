require_relative 'shared'

def send_data_file(req)
  data = File.read(req.param('path'))
  BenchmarkResponse.new(200, { 'Content-Type' => 'application/octet-stream' }, [data])
end
