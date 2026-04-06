require_relative 'shared'

# vuln-code-snippet start ruby_ssti_file_fixed
def render_from_file(req)
  name = req.param('name')
  template = File.read('templates/greeting.erb') # vuln-code-snippet safe-line ruby_ssti_file_fixed
  BenchmarkResponse.html(template.gsub('{{name}}', escape_html(name)))
end
# vuln-code-snippet end ruby_ssti_file_fixed
