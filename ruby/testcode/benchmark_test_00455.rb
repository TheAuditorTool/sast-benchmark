require_relative 'shared'
require 'cgi'

def handler(req)
  comment = req.post('comment')
  author = req.post('author')
  safe_comment = CGI.escapeHTML(comment)
  safe_author = CGI.escapeHTML(author)
  body = "<blockquote cite=\"#{safe_author}\">#{safe_comment}</blockquote>"
  BenchmarkResponse.html(body)
end
