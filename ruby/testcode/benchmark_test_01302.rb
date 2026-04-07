require_relative 'shared'

def display_error_page(req)
  message = escape_html(req.param('message'))
  BenchmarkResponse.html("<html><body><div class='alert'>#{message}</div></body></html>")
end
