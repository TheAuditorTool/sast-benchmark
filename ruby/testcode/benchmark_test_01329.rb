require_relative 'shared'

def read_template(req)
  template_name = req.param('template')
  content = File.read("/var/app/templates/#{template_name}.html")
  BenchmarkResponse.html(content)
end
