require_relative 'shared'

def render_notification(req)
  template_str = req.post('template')
  output = ERB.new(template_str).result
  BenchmarkResponse.html(output)
end
