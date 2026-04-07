require_relative 'shared'

def render_page_with_data(req)
  val = req.param('val')
  safe_json = val.to_json.gsub('<', '\u003c').gsub('>', '\u003e').gsub('&', '\u0026')
  html = "<script>var data = #{safe_json};</script>"
  BenchmarkResponse.html(html)
end
