require_relative 'shared'

def binread_file(req)
  data = IO.binread(req.param('file'))
  BenchmarkResponse.new(200, { 'Content-Type' => 'application/octet-stream' }, [data])
end
