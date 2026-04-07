require_relative 'shared'

def render_from_file(req)
  name = req.param('name')
  template = File.read('templates/greeting.erb')
  BenchmarkResponse.html(template.gsub('{{name}}', escape_html(name)))
end
