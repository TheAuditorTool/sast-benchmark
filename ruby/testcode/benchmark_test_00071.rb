require_relative 'shared'

def get_report(req)
  name = req.param('name')
  content = File.open("/data/reports/#{name}") { |f| f.read }
  BenchmarkResponse.ok(content)
end
