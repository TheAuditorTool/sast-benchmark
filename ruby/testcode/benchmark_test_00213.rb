require_relative 'shared'

def redirect_meta_refresh(req)
  html = "<meta http-equiv='refresh' content='0;url=#{req.param('url')}'>"
  BenchmarkResponse.html(html)
end
