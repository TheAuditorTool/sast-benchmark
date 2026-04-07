require_relative 'shared'

def normalize_input(val)
  val.to_s.strip
end

def render_notification(req)
  raw_msg = req.param('message')
  normalized = normalize_input(raw_msg)
  html = raw(normalized)
  BenchmarkResponse.html("<div class='notification'>#{html}</div>")
end
