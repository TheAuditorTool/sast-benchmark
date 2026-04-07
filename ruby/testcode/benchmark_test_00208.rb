require_relative 'shared'

def render_username_display(req)
  name = req.param('name')
  safe_name = ERB::Util.html_escape(name)
  BenchmarkResponse.html("<span class='username'>#{safe_name}</span>")
end
