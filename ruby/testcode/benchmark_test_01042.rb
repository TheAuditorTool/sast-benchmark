require_relative 'shared'

def render_custom_template(req)
  template_str = req.param('template')
  output = Tilt['erb'].new { template_str }.render
  BenchmarkResponse.html(output)
end
