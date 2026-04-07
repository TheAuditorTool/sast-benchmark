require_relative 'shared'

def fetch_export(req)
  export_name = req.param('export')
  data = IO.read("/tmp/exports/#{export_name}")
  BenchmarkResponse.ok(data)
end
