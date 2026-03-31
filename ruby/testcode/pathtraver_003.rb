require_relative 'shared'

# vuln-code-snippet start ruby_pt_file_open
def get_report(req)
  name = req.param('name')
  content = File.open("/data/reports/#{name}") { |f| f.read } # vuln-code-snippet vuln-line ruby_pt_file_open
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ruby_pt_file_open
